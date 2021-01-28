pub mod csv;
pub mod poi;

use crate::csv::{read_csv, write_csv, InputRecord, OutputRecord};
use anyhow::{Context, Result};
use osmpbfreader::OsmPbfReader;
use poi::{cal_relations, cal_ways, refine, OsmPbfReaderExt};
use rusqlite::{params, Connection, OpenFlags};
use std::path::{Path, PathBuf};

pub fn data_dir() -> Result<PathBuf> {
    // create data_dirs
    let mut path = dirs::data_dir().context("Cannot open application support folder")?;
    path.push("osmpoi");
    Ok(path)
}

pub fn add_osm_pbf<P: AsRef<Path>>(pbf_path: P) -> Result<()> {
    let pbf_path = pbf_path.as_ref();
    let stem = pbf_path.file_stem().context("Cannot get file stem")?;
    let db_path = data_dir()?.join(stem).with_extension("osm.poi");
    let r = std::fs::File::open(pbf_path)?;
    let mut pbf = OsmPbfReader::new(r);
    let mut conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    )?;
    let mut tx = conn.transaction()?;
    pbf.dump(&mut tx)?;
    println!("dump success");
    cal_ways(&mut tx)?;
    println!("calculate way success");
    cal_relations(&mut tx)?;
    println!("calculate relation success");
    refine(&mut tx)?;
    println!("refine success");
    println!("Finish!");
    tx.commit()?;
    Ok(())
}

pub fn dis_to_deg(dis: f32) -> f32 {
    (dis / 6_371_000. / std::f32::consts::PI * 180.) as f32
}

pub fn deg_to_deci(lat_or_lon: f32) -> i32 {
    (lat_or_lon * 10_000_000.) as i32
}
pub fn deci_to_deg(lat_or_lon: i32) -> f32 {
    (lat_or_lon as f32) / 10_000_000.
}

pub fn list_data_dir() -> Result<Vec<String>> {
    let entries = std::fs::read_dir(data_dir()?.as_path())?;
    let mut ret = Vec::new();
    for entry in entries {
        ret.push(
            entry?
                .file_name()
                .into_string()
                .map_err(|_| anyhow::Error::msg("Cannor get entry"))?,
        );
    }
    Ok(ret)
}

pub fn query_csv(csv_path: &str, csv_new_path: &str, name: &str, distance: f32) -> Result<()> {
    let db_path = data_dir()?.join(name);
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "SELECT poi_type, lat, lon, d_lat, d_lon, tags FROM poi 
         WHERE lat BETWEEN ?1 AND ?2
         AND lon BETWEEN ?3 AND ?4",
    )?;

    let inputs = read_csv(csv_path)?;
    let mut res = Vec::new();
    for i in inputs {
        let InputRecord { id, lat, lon } = i;
        let lat_lb = deg_to_deci(lat - dis_to_deg(distance));
        let lat_rt = deg_to_deci(lat + dis_to_deg(distance));
        let lon_lb = deg_to_deci(lon - dis_to_deg(distance));
        let lon_rt = deg_to_deci(lon + dis_to_deg(distance));
        let query_rows = stmt.query_map(params![lat_lb, lat_rt, lon_lb, lon_rt], |row| {
            Ok(OutputRecord {
                refer_id: id,
                poi_type: row.get(0)?,
                lat: deci_to_deg(row.get(1)?),
                lon: deci_to_deg(row.get(2)?),
                delta_lat: deci_to_deg(row.get(3)?),
                delta_lon: deci_to_deg(row.get(4)?),
                tags: row.get(5)?,
            })
        })?;
        for r in query_rows {
            let r = r?;
            res.push(r);
        }
    }
    write_csv(csv_new_path, res)?;
    Ok(())
}
