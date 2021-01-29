pub mod ffi;
pub mod poi;

use anyhow::Result;
use csv::{ReaderBuilder, WriterBuilder};
use derive_more::{Add, From, Sub};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

pub use ffi::*;
pub use poi::*;

pub fn query_csv(
    input_path: &str,
    output_path: &str,
    dataset_path: &str,
    distance: f32,
    strict: bool,
) -> Result<()> {
    let conn = Connection::open(dataset_path)?;
    // choose different select statement according to the strict flag
    let mut stmt = if strict {
        conn.prepare(
            // in script mode, the center of poi must be inside your selecting box
            "SELECT poi_type, lat, lon, d_lat, d_lon, tags FROM poi WHERE lat BETWEEN ?1 AND ?2 AND lon BETWEEN ?3 AND ?4"
        )?
    } else {
        conn.prepare(
            // in non-strict mode, the poi box has intersection with your selecting box
            "SELECT poi_type, lat, lon, d_lat, d_lon, tags FROM poi WHERE NOT ((lat - d_lat > ?2) OR (lat + d_lat < ?1) OR (lon - d_lon > ?4) OR (lon + d_lon < ?3))",
        )?
    };
    let inputs = read_csv(input_path)?;
    let mut res = Vec::new();
    for i in inputs {
        let InputRecord { id, lat, lon } = i;
        let distance: Distance = distance.into();
        let lat_bottom: DecimicroLatitude = (lat - distance.to_full_lat()).into();
        let lon_left: DecimicroLongitude = (lon - distance.to_full_lon()).into();
        let lat_top: DecimicroLatitude = (lat + distance.to_full_lat()).into();
        let lon_right: DecimicroLongitude = (lon + distance.to_full_lon()).into();
        let query_rows = stmt.query_map(
            params![lat_bottom.0, lat_top.0, lon_left.0, lon_right.0],
            |row| {
                // directly construct OutputRecord
                Ok(OutputRecord {
                    refer_id: id,
                    poi_type: row.get(0)?,
                    lat: DecimicroLatitude(row.get(1)?).into(),
                    lon: DecimicroLongitude(row.get(2)?).into(),
                    delta_lat: DecimicroLatitude(row.get(3)?).into(),
                    delta_lon: DecimicroLongitude(row.get(4)?).into(),
                    distance: Distance::calculate(
                        DecimicroLatitude(row.get(1)?).into(),
                        DecimicroLongitude(row.get(2)?).into(),
                        lat,
                        lon,
                    ),
                    tags: row.get(5)?,
                })
            },
        )?;
        for r in query_rows {
            let r: OutputRecord = r?;
            if strict {
                if r.distance.0 > distance.0 {
                    continue;
                }
            }

            res.push(r);
        }
    }
    write_csv(output_path, res)?;
    Ok(())
}

/// Latitude in decimicro degree (10^-7)
#[derive(Debug, Add, Sub, From, Copy, Clone)]
pub struct DecimicroLatitude(i32);

#[derive(Debug, Add, Sub, From, Copy, Clone)]
pub struct DecimicroLongitude(i32);

#[derive(Debug, Add, Sub, Serialize, Deserialize, From, Copy, Clone)]
#[serde(transparent)]
pub struct Latitude(f32);

#[derive(Debug, Add, Sub, Serialize, Deserialize, From, Copy, Clone)]
#[serde(transparent)]
pub struct Longitude(f32);

impl From<DecimicroLatitude> for Latitude {
    fn from(item: DecimicroLatitude) -> Self {
        Self((item.0 as f32) / 10_000_000.)
    }
}

impl From<DecimicroLongitude> for Longitude {
    fn from(item: DecimicroLongitude) -> Self {
        Self((item.0 as f32) / 10_000_000.)
    }
}

impl From<Latitude> for DecimicroLatitude {
    fn from(item: Latitude) -> Self {
        Self((item.0 * 10_000_000.) as i32)
    }
}

impl From<Longitude> for DecimicroLongitude {
    fn from(item: Longitude) -> Self {
        Self((item.0 * 10_000_000.) as i32)
    }
}

#[derive(Debug, Add, Sub, Serialize, Deserialize, From, Copy, Clone)]
#[serde(transparent)]
pub struct Distance(f32);

impl Distance {
    /// Convert the distance (kilometer) to full latitude (degree)
    pub fn to_full_lat(&self) -> Latitude {
        Latitude(self.0 / RADIUS_EARTH / PI * 180.)
    }
    /// Convert the distance (kilometer) to full longitude (degree)
    pub fn to_full_lon(&self) -> Longitude {
        Longitude(self.0 / RADIUS_EARTH / PI * 180.)
    }
    /// Calculate the actual distance (kilometer) of two points (degress)
    pub fn calculate(lat1: Latitude, lon1: Longitude, lat2: Latitude, lon2: Longitude) -> Self {
        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;
        let a = deg_to_rad(dlat.0 / 2.).sin().powi(2)
            + deg_to_rad(lat1.0).cos()
                * deg_to_rad(lat2.0).cos()
                * deg_to_rad(dlon.0 / 2.).sin().powi(2);
        let c = 2. * a.sqrt().asin();
        Distance(c * RADIUS_EARTH)
    }
}

/// The radius of the Earth
static RADIUS_EARTH: f32 = 6_371.;

/// convert degree to radian
fn deg_to_rad(deg: f32) -> f32 {
    deg / 180. * PI
}

// /// convert radian to degree
// fn rad_to_deg(rad: f32) -> f32 {
//     rad / PI * 180.
// }

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    pub id: i64,
    pub lat: Latitude,
    pub lon: Longitude,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    pub refer_id: i64,
    pub poi_type: i32,
    pub lat: Latitude,
    pub lon: Longitude,
    pub delta_lat: Latitude,
    pub delta_lon: Longitude,
    pub distance: Distance,
    pub tags: String,
}

/// return records from the csv
pub fn read_csv(input_path: &str) -> Result<Vec<InputRecord>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)?;
    let mut ret = Vec::new();
    for record in rdr.deserialize() {
        let record: InputRecord = record?;
        ret.push(record);
    }
    Ok(ret)
}

/// write csv to new file with output
pub fn write_csv(output_path: &str, outputs: Vec<OutputRecord>) -> Result<()> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;
    for output in outputs {
        wtr.serialize(output)?;
    }
    Ok(())
}
