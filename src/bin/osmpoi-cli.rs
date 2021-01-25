use ::osmpoi::poi::{cal_relations, cal_ways, refine, OsmPbfReaderExt};
use anyhow::{Context, Result};
use osmpbfreader::OsmPbfReader;
use rusqlite::{Connection, OpenFlags};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let pbf_path = args.get(1).context("Fail to get pbf path")?;
    let db_path = args.get(2).context("Fail to get db path")?;
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
