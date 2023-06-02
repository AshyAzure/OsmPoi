use ormlite::model::*;
use ormlite::sqlite::SqliteConnection;
use snafu::{prelude::*, Whatever};

#[derive(Model, Clone)]
pub struct Relation {
    pub id: i64,
}

const CREATE_TABLE_RELATION: &str = "
    CREATE TABLE IF NOT EXISTS relation (
        id INTEGER
    )";

const DROP_TABLE_RELATION: &str = "DROP TABLE relation";

impl Relation {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(CREATE_TABLE_RELATION)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to create table relation")?;
        Ok(())
    }

    pub async fn drop_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(DROP_TABLE_RELATION)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to drop table relation")?;
        Ok(())
    }
}

#[derive(Model, Clone)]
pub struct Membership {
    pub id: i64,
    pub mid: i64,
    pub is_relation: bool,
}

const CREATE_TABLE_MEMBERSHIP: &str = "
    CREATE TABLE IF NOT EXISTS membership (
        id INTEGER,
        mid INTEGER,
        is_relation BOOLEAN
    )";

const CREATE_INDEX_MEMBERSHIP_ID: &str = "CREATE INDEX membership_id_index on membership (mid)";
const CREATE_INDEX_MEMBERSHIP_MID: &str = "CREATE INDEX membership_mid_index on membership (mid)";
const CREATE_INDEX_MEMBERSHIP_RELATION: &str =
    "CREATE INDEX membership_relation_index on membership (is_relation)";

const DROP_TABLE_MEMBERSHIP: &str = "DROP TABLE IF EXISTS membership";

impl Membership {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(CREATE_TABLE_MEMBERSHIP)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to create table membership")?;
        ormlite::query(CREATE_INDEX_MEMBERSHIP_ID)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to create index id")?;
        ormlite::query(CREATE_INDEX_MEMBERSHIP_MID)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to create index mid")?;
        ormlite::query(CREATE_INDEX_MEMBERSHIP_RELATION)
            .execute(&mut *conn)
            .await
            .whatever_context("Fail to create index relation")?;
        Ok(())
    }
    pub async fn drop_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(DROP_TABLE_MEMBERSHIP)
            .execute(conn)
            .await
            .whatever_context("Fail to drop table required_relation")?;
        Ok(())
    }
}

#[derive(Model, Clone)]
pub struct Position {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub weight: f64,
}

const CREATE_TABLE_POSITION: &str = "
    CREATE TABLE IF NOT EXISTS position (
        id INTEGER PRIMARY KEY,
        lat REAL,
        lon REAL,
        weight REAL
    )";

impl Position {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(CREATE_TABLE_POSITION)
            .execute(conn)
            .await
            .whatever_context("Fail to create table position")?;
        Ok(())
    }
}

#[derive(Model, Clone)]
pub struct Tag {
    pub id: i64,
    pub key: String,
    pub value: String,
}

const CREATE_TABLE_TAG: &str = "
    CREATE TABLE IF NOT EXISTS tag (
        id INTEGER,
        key TEXT,
        value TEXT
    )";

impl Tag {
    pub async fn create_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(CREATE_TABLE_TAG)
            .execute(conn)
            .await
            .whatever_context("Fail to create table tag")?;
        Ok(())
    }
    pub async fn drop_table(conn: &mut SqliteConnection) -> Result<(), Whatever> {
        ormlite::query(DROP_TABLE_TAG)
            .execute(conn)
            .await
            .whatever_context("Fail to drop table tag")?;
        Ok(())
    }
}
