use anyhow::Result;
use rusqlite::{CachedStatement, Transaction, NO_PARAMS};

pub fn create_nodes(tx: &Transaction) -> Result<()> {
    tx.execute(
        "CREATE TABLE nodes (
            node_id  INTEGER PRIMARY KEY NOT NULL CHECK(node_id >= 0),
            lat      INTEGER NOT NULL CHECK(lat BETWEEN -900000000 AND 900000000),
            lon      INTEGER NOT NULL CHECK(lat BETWEEN -1800000000 AND 1800000000),
            has_name INTEGER NOT NULL,
            tags     TEXT
        );",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn create_ways(tx: &Transaction) -> Result<()> {
    tx.execute(
        "CREATE TABLE ways (
            way_id   INTEGER PRIMARY KEY NOT NULL CHECK(way_id >= 0),
            lat_lb   INTEGER NOT NULL CHECK(lat_lb BETWEEN -900000000 AND 900000000) DEFAULT 0,
            lon_lb   INTEGER NOT NULL CHECK(lon_lb BETWEEN -1800000000 AND 1800000000) DEFAULT 0,
            lat_rt   INTEGER NOT NULL CHECK(lat_rt BETWEEN lat_lb AND 900000000) DEFAULT 0,
            lon_rt   INTEGER NOT NULL CHECK(lon_rt BETWEEN lon_lb AND 1800000000) DEFAULT 0,
            has_name INTEGER NOT NULL,
            tags   TEXT
        );",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn create_way_nodes(tx: &Transaction) -> Result<()> {
    tx.execute(
        "CREATE TABLE way_nodes (
            way_id  INTEGER NOT NULL CHECK(way_id >= 0),
            node_id INTEGER NOT NULL CHECK(node_id >= 0)
        );",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE INDEX way_nodes_index ON way_nodes (way_id)",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn create_relationss(tx: &Transaction) -> Result<()> {
    tx.execute(
        "CREATE TABLE relations (
            relation_id INTEGER PRIMARY KEY NOT NULL CHECK(relation_id >= 0),
            lat_lb      INTEGER NOT NULL CHECK(lat_lb BETWEEN -900000000 AND 900000000) DEFAULT 0,
            lon_lb      INTEGER NOT NULL CHECK(lon_lb BETWEEN -1800000000 AND 1800000000) DEFAULT 0,
            lat_rt      INTEGER NOT NULL CHECK(lat_rt BETWEEN lat_lb AND 900000000) DEFAULT 0,
            lon_rt      INTEGER NOT NULL CHECK(lon_rt BETWEEN lon_lb AND 1800000000) DEFAULT 0,
            dep         INTEGER NOT NULL CHECK(dep >= 0) DEFAULT 0,
            has_name    INTEGER NOT NULL,
            tags        TEXT
        );",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn create_relation_references_index(tx: &Transaction) -> Result<()> {
    tx.execute(
        "CREATE TABLE relation_references (
            relation_id    INTEGER NOT NULL CHECK(relation_id >= 0),
            reference_id   INTEGER NOT NULL CHECK(reference_id >= 0),
            reference_type INTEGER NOT NULL CHECK(reference_type BETWEEN 0 AND 2)
        );",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE INDEX relation_references_index ON relation_references (relation_id)",
        NO_PARAMS,
    )?;
    Ok(())
}

macro_rules! prepare {
    ($func_name:ident, $sql:expr) => {
        pub fn $func_name<'a>(tx: &'a Transaction) -> rusqlite::Result<CachedStatement<'a>> {
            tx.prepare_cached($sql)
        }
    };
}

prepare!(
    prepare_insert_nodes,
    "INSERT INTO nodes (node_id, lat, lon, has_name, tags) VALUES (?1, ?2, ?3, ?4, ?5);"
);

prepare!(
    prepare_insert_ways,
    "INSERT INTO ways (way_id, has_name,tags) VALUES (?1, ?2, ?3)"
);

prepare!(
    prepare_insert_way_nodes,
    "INSERT INTO way_nodes (way_id, node_id) VALUES (?1, ?2)"
);

prepare!(
    prepare_insert_relations,
    "INSERT INTO relations (relation_id, has_name, tags) VALUES (?1, ?2, ?3)"
);

prepare!(
    prepare_insert_relation_references,
    "INSERT INTO relation_references (relation_id, reference_id, reference_type) VALUES (?1, ?2, ?3)"
);
