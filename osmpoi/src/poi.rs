#![allow(clippy::many_single_char_names)]
use anyhow::{Context, Result};
use osmpbfreader::{
    objects::{OsmId, OsmObj, Tags},
    OsmPbfReader,
};
use rusqlite::{params, Transaction, Connection, NO_PARAMS, OpenFlags};
use serde_json::Map;
use std::cmp::{max, min};
use std::fs::File;

mod sql;
use sql::*;

/// A struct that represents the counts of different elements in a file.
#[derive(Debug)]
pub struct OsmCount {
    pub node: u64,
    pub way: u64,
    pub relation: u64,
}

impl OsmCount {
    /// Create a new count object with all fields set to 0.
    fn new() -> Self {
        Self {
            node: 0,
            way: 0,
            relation: 0,
        }
    }
    /// Increase the count according to the type of obj it reads.
    fn incr(&mut self, obj: OsmObj) {
        match obj {
            OsmObj::Node(_) => self.node += 1,
            OsmObj::Way(_) => self.way += 1,
            OsmObj::Relation(_) => self.relation += 1,
        }
    }
}

fn pbf_reader_from_path(path: &str) -> Result<OsmPbfReader<File>> {
    let read = File::open(path)?;
    Ok(OsmPbfReader::new(read))
}

pub fn count(path: &str) -> Result<OsmCount> {
    let mut reader = pbf_reader_from_path(path)?;
    let mut count = OsmCount::new();
    for obj in reader.par_iter() {
        match obj {
            Ok(obj) => count.incr(obj),
            Err(err) => {
                return Err(err).context("Fail to read par_iter objects");
            }
        }
    }
    Ok(count)
}

pub fn dump(pbf_path: &str, db_path: &str) -> Result<()> {
    // init the pbf reader
    let mut reader = pbf_reader_from_path(pbf_path)?;

    let mut conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    )?;
    let tx = conn.transaction()?;
    // create databases
    sql::create_nodes(&tx)?;
    sql::create_ways(&tx)?;
    sql::create_way_nodes(&tx)?;
    sql::create_relationss(&tx)?;
    sql::create_relation_references_index(&tx)?;
    // dump file
    let mut insert_nodes_stmt = sql::prepare_insert_nodes(&tx)?;
    let mut insert_ways_stmt = sql::prepare_insert_ways(&tx)?;
    let mut insert_way_nodes_stmt = sql::prepare_insert_way_nodes(&tx)?;
    let mut insert_relations_stmt = sql::prepare_insert_relations(&tx)?;
    let mut insert_relation_references_stmt = sql::prepare_insert_relation_references(&tx)?;
    for obj in reader.par_iter() {
        match obj {
            Ok(obj) => match obj {
                OsmObj::Node(n) => {
                    insert_nodes_stmt.execute(params![
                        n.id.0,
                        n.decimicro_lat,
                        n.decimicro_lon,
                        tags_has_name(n.tags.clone()),
                        tags_to_json_string(n.tags.clone()),
                    ])?;
                }
                OsmObj::Way(w) => {
                    insert_ways_stmt.execute(params![
                        w.id.0,
                        tags_has_name(w.tags.clone()),
                        tags_to_json_string(w.tags.clone())
                    ])?;
                    for node_id in w.nodes {
                        insert_way_nodes_stmt.execute(params![w.id.0, node_id.0])?;
                    }
                }
                OsmObj::Relation(r) => {
                    insert_relations_stmt.execute(params![
                        r.id.0,
                        tags_has_name(r.tags.clone()),
                        tags_to_json_string(r.tags.clone())
                    ])?;
                    for reference in r.refs {
                        let member = reference.member;
                        insert_relation_references_stmt.execute(params![
                            r.id.0,
                            member.inner_id(),
                            match member {
                                OsmId::Node(_) => 0,
                                OsmId::Way(_) => 1,
                                OsmId::Relation(_) => 2,
                            }
                        ])?;
                    }
                }
            },
            Err(err) => {
                return Err(err).context("Fail to read par_iter objects");
            }
        }
    }
    Ok(())
}

/// A light extension to the OsmPbfReader
pub trait OsmPbfReaderExt {
    /// Returns the count of the pbf file,
    /// should return Err when the reading process fails,
    /// or when the rewind fails after use.
    fn count(&mut self) -> Result<OsmCount>;
    fn dump(&mut self, tx: &mut Transaction) -> Result<()>;
}

impl OsmPbfReaderExt for OsmPbfReader<File> {
    fn count(&mut self) -> Result<OsmCount> {
        let mut count = OsmCount::new();
        for obj in self.par_iter() {
            match obj {
                Ok(obj) => count.incr(obj),
                Err(err) => {
                    self.rewind()?;
                    return Err(err).context("Fail to read par_iter objects");
                }
            }
        }
        // count success and return
        self.rewind()?;
        Ok(count)
    }
    fn dump(&mut self, tx: &mut Transaction) -> Result<()> {
        // create databases
        sql::create_nodes(&tx)?;
        sql::create_ways(&tx)?;
        sql::create_way_nodes(&tx)?;
        sql::create_relationss(&tx)?;
        sql::create_relation_references_index(&tx)?;
        // dump file
        let mut insert_nodes_stmt = sql::prepare_insert_nodes(&tx)?;
        let mut insert_ways_stmt = sql::prepare_insert_ways(&tx)?;
        let mut insert_way_nodes_stmt = sql::prepare_insert_way_nodes(&tx)?;
        let mut insert_relations_stmt = sql::prepare_insert_relations(&tx)?;
        let mut insert_relation_references_stmt = sql::prepare_insert_relation_references(&tx)?;
        for obj in self.par_iter() {
            match obj {
                Ok(obj) => match obj {
                    OsmObj::Node(n) => {
                        insert_nodes_stmt.execute(params![
                            n.id.0,
                            n.decimicro_lat,
                            n.decimicro_lon,
                            tags_has_name(n.tags.clone()),
                            tags_to_json_string(n.tags.clone()),
                        ])?;
                    }
                    OsmObj::Way(w) => {
                        insert_ways_stmt.execute(params![
                            w.id.0,
                            tags_has_name(w.tags.clone()),
                            tags_to_json_string(w.tags.clone())
                        ])?;
                        for node_id in w.nodes {
                            insert_way_nodes_stmt.execute(params![w.id.0, node_id.0])?;
                        }
                    }
                    OsmObj::Relation(r) => {
                        insert_relations_stmt.execute(params![
                            r.id.0,
                            tags_has_name(r.tags.clone()),
                            tags_to_json_string(r.tags.clone())
                        ])?;
                        for reference in r.refs {
                            let member = reference.member;
                            insert_relation_references_stmt.execute(params![
                                r.id.0,
                                member.inner_id(),
                                match member {
                                    OsmId::Node(_) => 0,
                                    OsmId::Way(_) => 1,
                                    OsmId::Relation(_) => 2,
                                }
                            ])?;
                        }
                    }
                },
                Err(err) => {
                    self.rewind()?;
                    return Err(err).context("Fail to read par_iter objects");
                }
            }
        }
        Ok(())
    }
}

/// Turn the tags of way and relations into json string
fn tags_to_json_string(tags: Tags) -> String {
    let mut tag_map = Map::new();
    for (k, v) in tags.iter() {
        tag_map.insert(
            format!("{}", k),
            serde_json::Value::String(format!("{}", v)),
        );
    }
    serde_json::Value::Object(tag_map).to_string()
}

/// If the tags key contains name
fn tags_has_name(tags: Tags) -> bool {
    for (k, _) in tags.iter() {
        if format!("{}", k).contains("name") {
            return true;
        }
    }
    false
}
type NullableI32 = Option<i32>;
type NullableLatLon = (NullableI32, NullableI32, NullableI32, NullableI32);
/// if all null return Err
fn nullable_min(a: NullableI32, b: NullableI32, c: NullableI32) -> Result<i32> {
    match (a, b, c) {
        (None, None, None) => Err(anyhow::Error::msg("all null")),
        (Some(x), None, None) | (None, Some(x), None) | (None, None, Some(x)) => Ok(x),
        (Some(x), Some(y), None) | (None, Some(x), Some(y)) | (Some(x), None, Some(y)) => {
            Ok(min(x, y))
        }
        (Some(x), Some(y), Some(z)) => Ok(min(min(x, y), z)),
    }
}
fn nullable_max(a: NullableI32, b: NullableI32, c: NullableI32) -> Result<i32> {
    match (a, b, c) {
        (None, None, None) => Err(anyhow::Error::msg("all null")),
        (Some(x), None, None) | (None, Some(x), None) | (None, None, Some(x)) => Ok(x),
        (Some(x), Some(y), None) | (None, Some(x), Some(y)) | (Some(x), None, Some(y)) => {
            Ok(max(x, y))
        }
        (Some(x), Some(y), Some(z)) => Ok(max(max(x, y), z)),
    }
}
/// calculate the lat and lon range of the way
pub fn cal_ways(tx: &mut Transaction) -> Result<()> {
    // this step use all the nodes in a way to calculate the max and min lats and lons
    let mut all_ways = tx.prepare(SQL_SELECT_ALL_WAY_ID)?;
    let mut query_way = tx.prepare(SQL_SELECT_LAT_LON_WITH_WAY_ID)?;
    let mut update_way = tx.prepare(SQL_UPDATE_WAY_WITH_LAT_LON)?;
    let way_id_rows = all_ways.query_map(NO_PARAMS, |row| row.get(0))?;
    for way_id in way_id_rows {
        let way_id: i64 = way_id?;
        let (lat_lb, lat_rt, lon_lb, lon_rt): (i32, i32, i32, i32) = query_way
            .query_row(params![way_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?;
        update_way.execute(params![lat_lb, lon_lb, lat_rt, lon_rt, way_id])?;
    }

    Ok(())
}

/// calculate the lat and lon range of the relation
pub fn cal_relations(tx: &mut Transaction) -> Result<()> {
    let mut all_relations = tx.prepare(SQL_SELECT_RELATIONS_YET_UNDETERMINED)?;
    let mut update_relation = tx.prepare(SQL_UPDATE_RELATIONS)?;
    let mut check_dep = tx.prepare(SQL_SELECT_UNDETERMINED_RELATION_DEPENDENCIES)?;
    let mut query_nodes = tx.prepare(SQL_SELECT_LAT_LON_FROM_NODES_WITH_RELATION_ID)?;
    let mut query_ways = tx.prepare(SQL_SELECT_LAT_LON_FROM_WAYS_WITH_RELATION_ID)?;
    let mut query_relations = tx.prepare(SQL_SELECT_LAT_LON_FROM_RELATIONS_WITH_RELATION_ID)?;
    // println!("build success");
    loop {
        let mut change_flag = false;
        let relation_id_rows = all_relations.query_map(NO_PARAMS, |row| row.get(0))?;
        for relation_id in relation_id_rows {
            let relation_id: i64 = relation_id?;
            let deps: i32 = check_dep.query_row(params![relation_id], |row| row.get(0))?;
            // skip this if any dep is not set
            if deps > 0 {
                continue;
            }

            let (lat_lb_n, lat_rt_n, lon_lb_n, lon_rt_n): NullableLatLon = query_nodes
                .query_row(params![relation_id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
                })
                .context("6")?;
            let (lat_lb_w, lat_rt_w, lon_lb_w, lon_rt_w): NullableLatLon = query_ways
                .query_row(params![relation_id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
                })
                .context("7")?;
            let (lat_lb_r, lat_rt_r, lon_lb_r, lon_rt_r): NullableLatLon = query_relations
                .query_row(params![relation_id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
                })
                .context("8")?;

            let lat_lb = nullable_min(lat_lb_n, lat_lb_w, lat_lb_r)?;
            let lon_lb = nullable_min(lon_lb_n, lon_lb_w, lon_lb_r)?;
            let lat_rt = nullable_max(lat_rt_n, lat_rt_w, lat_rt_r)?;
            let lon_rt = nullable_max(lon_rt_n, lon_rt_w, lon_rt_r)?;
            update_relation.execute(params![lat_lb, lon_lb, lat_rt, lon_rt, relation_id])?;
            change_flag = true;
        }
        // if no new relation added break from the infinite loop
        if !change_flag {
            break;
        }
    }
    if let Ok(v) = tx.query_row(sql::SQL_COUNT_UNDETERMINED_RELATIONS, NO_PARAMS, |row| {
        row.get::<_, i32>(0)
    }) {
        if v == 0 {
            Ok(())
        } else {
            Err(anyhow::Error::msg("Some problems in relation dependencies"))
        }
    } else {
        Err(anyhow::Error::msg("Some problems in relation dependencies"))
    }
}

/// refine the databases
pub fn refine(tx: &mut Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_POI, NO_PARAMS)?;

    tx.execute(SQL_INSERT_INTO_POI_FROM_NODES, NO_PARAMS)?;
    tx.execute(SQL_INSERT_INTO_POI_FROM_WAYS, NO_PARAMS)?;
    tx.execute(SQL_INSERT_INTO_POI_FROM_RELATIONS, NO_PARAMS)?;
    tx.execute(SQL_DROP_TABLE_NODES, NO_PARAMS)?;
    tx.execute(SQL_DROP_TABLE_WAYS, NO_PARAMS)?;
    tx.execute(SQL_DROP_TABLE_WAY_NODES, NO_PARAMS)?;
    tx.execute(SQL_DROP_TABLE_RELATIONS, NO_PARAMS)?;
    tx.execute(SQL_DROP_TABLE_RELATION_REFERENCES, NO_PARAMS)?;
    Ok(())
}
