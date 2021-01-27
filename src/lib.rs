pub mod csv;
pub mod poi;

use anyhow::{Context, Result};
use osmpbfreader::OsmPbfReader;
use poi::{cal_relations, cal_ways, refine, OsmPbfReaderExt};
use rusqlite::{Connection, OpenFlags};
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

pub fn dis_to_deg(dis: f32) -> i32 {
    (dis / 6_371_000. / std::f32::consts::PI * 180.) as i32
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
