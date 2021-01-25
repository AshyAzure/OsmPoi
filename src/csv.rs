use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    id: i64,
    lat: f32,
    lon: f32,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    refer_id: i64,
    distance: f32,
    lat: f32,
    lon: f32,
    delta_lat: f32,
    delta_lon: f32,
}

pub fn read_csv(path: &str) -> Result<Vec<InputRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(path)?);
    let mut ret = Vec::new();
    for record in rdr.deserialize() {
        let record: InputRecord = record?;
        ret.push(record);
    }
    Ok(ret)
}
