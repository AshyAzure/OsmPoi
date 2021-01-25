use anyhow::Result;
use rusqlite::{CachedStatement, Transaction, NO_PARAMS};

macro_rules! include_sqls {
    ($($name:ident => $path:literal), +) => {
        $(pub static $name: &str = std::include_str!($path);)+
    };
}

macro_rules! fn_tx_prepare_cached {
    ($($func_name:ident => $sql:expr),+) => {
        $(
        pub fn $func_name<'a>(tx: &'a Transaction) -> rusqlite::Result<CachedStatement<'a>> {
            tx.prepare_cached($sql)
        }
        )+
    };
}

// include all the sql files
include_sqls! {
    SQL_CREATE_TABLE_NODES => "create_table_nodes.sql",
    SQL_CREATE_TABLE_WAYS => "create_table_ways.sql",
    SQL_CREATE_TABLE_WAY_NODES => "create_table_way_nodes.sql",
    SQL_CREATE_INDEX_WAY_NODES_INDEX => "create_index_way_nodes_index.sql",
    SQL_CREATE_TABLE_RELATIONS => "create_table_relations.sql",
    SQL_CREATE_TABLE_RELATION_REFERENCES => "create_table_relation_references.sql",
    SQL_CREATE_INDEX_RELATION_REFERENCES_INDEX => "create_index_relation_references_index.sql",
    SQL_INSERT_INTO_NODES => "insert_into_nodes.sql",
    SQL_INSERT_INTO_WAYS => "insert_into_ways.sql",
    SQL_INSERT_INTO_WAY_NODES => "insert_into_way_nodes.sql",
    SQL_INSERT_INTO_RELATIONS => "insert_into_relations.sql",
    SQL_INSERT_INTO_RELATION_REFERENCES => "insert_into_relation_references.sql",
    SQL_SELECT_ALL_WAY_ID => "select_all_way_id.sql",
    SQL_SELECT_LAT_LON_WITH_WAY_ID => "select_lat_lon_with_way_id.sql",
    SQL_UPDATE_WAY_WITH_LAT_LON => "update_way_with_lat_lon.sql",
    SQL_SELECT_RELATIONS_YET_UNDETERMINED => "select_relations_yet_undetermined.sql",
    SQL_UPDATE_RELATIONS => "update_relations.sql",
    SQL_SELECT_UNDETERMINED_RELATION_DEPENDENCIES => "select_undetermined_relation_dependencies.sql",
    SQL_SELECT_LAT_LON_FROM_NODES_WITH_RELATION_ID => "select_lat_lon_from_nodes_with_relation_id.sql",
    SQL_SELECT_LAT_LON_FROM_WAYS_WITH_RELATION_ID => "select_lat_lon_from_ways_with_relation_id.sql",
    SQL_SELECT_LAT_LON_FROM_RELATIONS_WITH_RELATION_ID => "select_lat_lon_from_relations_with_relation_id.sql",
    SQL_COUNT_UNDETERMINED_RELATIONS => "count_undetermined_relations.sql",
    SQL_CREATE_TABLE_POI => "create_table_poi.sql",
    SQL_INSERT_INTO_POI_FROM_NODES => "insert_into_poi_from_nodes.sql",
    SQL_INSERT_INTO_POI_FROM_WAYS => "insert_into_poi_from_ways.sql",
    SQL_INSERT_INTO_POI_FROM_RELATIONS => "insert_into_poi_from_relations.sql",
    SQL_DROP_TABLE_NODES => "drop_table_nodes.sql",
    SQL_DROP_TABLE_WAYS => "drop_table_ways.sql",
    SQL_DROP_TABLE_WAY_NODES => "drop_table_way_nodes.sql",
    SQL_DROP_TABLE_RELATIONS => "drop_table_relations.sql",
    SQL_DROP_TABLE_RELATION_REFERENCES => "drop_table_relation_references.sql"
}

pub fn create_nodes(tx: &Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_NODES, NO_PARAMS)?;
    Ok(())
}

pub fn create_ways(tx: &Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_WAYS, NO_PARAMS)?;
    Ok(())
}

pub fn create_way_nodes(tx: &Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_WAY_NODES, NO_PARAMS)?;
    tx.execute(SQL_CREATE_INDEX_WAY_NODES_INDEX, NO_PARAMS)?;
    Ok(())
}

pub fn create_relationss(tx: &Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_RELATIONS, NO_PARAMS)?;
    Ok(())
}

pub fn create_relation_references_index(tx: &Transaction) -> Result<()> {
    tx.execute(SQL_CREATE_TABLE_RELATION_REFERENCES, NO_PARAMS)?;
    tx.execute(SQL_CREATE_INDEX_RELATION_REFERENCES_INDEX, NO_PARAMS)?;
    Ok(())
}

fn_tx_prepare_cached! {
    prepare_insert_nodes => SQL_INSERT_INTO_NODES,
    prepare_insert_ways => SQL_INSERT_INTO_WAYS,
    prepare_insert_way_nodes => SQL_INSERT_INTO_WAY_NODES,
    prepare_insert_relations => SQL_INSERT_INTO_RELATIONS,
    prepare_insert_relation_references => SQL_INSERT_INTO_RELATION_REFERENCES
}
