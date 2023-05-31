use anyhow::Context;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{Sqlite, SqlitePool, SqlitePoolOptions},
};
use std::path::Path;

/// Create a sqlite pool with the sqlite database specified with the path, which will 
/// fail when there is no such a file.
pub async fn sql_pool_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<SqlitePool> {
    Ok(SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&sqlite_url_from_path(path)?)
        .await?)
}

/// Create a new sqlite database from the path.
pub async fn sqlite_create_database<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    Ok(Sqlite::create_database(&sqlite_url_from_path(path)?).await?)
}

/// Check if sqlite database from the path exists.
pub async fn sqlite_database_exists<P: AsRef<Path>>(path: P) -> anyhow::Result<bool> {
    Ok(Sqlite::database_exists(&sqlite_url_from_path(path)?).await?)
}

/// Drop a sqlite database from the path.
pub async fn sqlite_drop_database<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    Ok(Sqlite::drop_database(&sqlite_url_from_path(path)?).await?)
}

/// Convert the path to a url, unchecked whether there really exists a 
/// databse file.
fn sqlite_url_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    Ok(format!(
        "sqlite:///{}",
        path.as_ref()
            .to_str()
            .context("The input path is invalid")?
    ))
}
