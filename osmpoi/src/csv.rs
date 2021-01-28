use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    pub id: i64,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    pub refer_id: i64,
    pub poi_type: i32,
    pub lat: f32,
    pub lon: f32,
    pub delta_lat: f32,
    pub delta_lon: f32,
    pub tags: String,
}

pub fn read_csv(path: &str) -> Result<Vec<InputRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut ret = Vec::new();
    for record in rdr.deserialize() {
        let record: InputRecord = record?;
        ret.push(record);
    }
    Ok(ret)
}

pub fn write_csv(path: &str, outputs: Vec<OutputRecord>) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    for o in outputs {
        wtr.serialize(o)?;
    }
    Ok(())
}
