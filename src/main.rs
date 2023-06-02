mod aggregate;
mod clean;
mod extract;
mod orm;
mod distance;

use crate::aggregate::aggregate_positions;
use crate::clean::clean_database;
use crate::extract::extract_required;
use crate::distance::dump_distances;
use ormlite::{sqlite::SqliteConnection, Connection};
use osmpbfreader::OsmPbfReader;
use snafu::{prelude::*, Whatever};
use std::fs::File;
use tokio::fs::rename;

#[tokio::main]
pub async fn main() -> Result<(), Whatever> {
    // paths
    let path = get_path()?;
    let db_path = format!("{}.db", path);
    let db_tmp_path = format!("{}.tmp", db_path);
    // create osm reader
    let mut reader = read_pbf(&path)?;
    // connect db
    let conn_string = format!("sqlite://{}?mode=rwc", db_tmp_path);
    let mut conn = SqliteConnection::connect(&conn_string)
        .await
        .whatever_context("Fail to connect to database")?;
    // run steps
    println!("Start extracting required information...");
    extract_required(&mut reader, &mut conn).await?;
    drop(reader);
    println!("Start aggregating relation positions...");
    aggregate_positions(&mut conn).await?;
    println!("Start dumping distances");
    dump_distances(&mut conn).await?;
    println!("Cleaning database...");
    clean_database(&mut conn).await?;
    conn.close()
        .await
        .whatever_context("Fail to close databse")?;
    rename(db_tmp_path, db_path)
        .await
        .whatever_context("Fail to rename temp db file")?;
    println!("Finish!");
    Ok(())
}

/// Get PBF file path from CLI options.
fn get_path() -> Result<String, Whatever> {
    let args: Vec<_> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .whatever_context("Fail to get path from CLI option")?;
    Ok(path)
}

fn read_pbf(path: &str) -> Result<OsmPbfReader<File>, Whatever> {
    let file = File::open(&path).whatever_context(format!("Fail to open file {:?}", path))?;
    let reader = OsmPbfReader::new(file);
    Ok(reader)
}
