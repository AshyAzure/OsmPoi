use crate::orm::{Membership, Position, Relation};
use ormlite::sqlite::SqliteConnection;
use snafu::Whatever;

pub async fn clean_database(conn: &mut SqliteConnection) -> Result<(), Whatever> {
    Position::drop_table(&mut *conn).await?;
    Relation::drop_table(&mut *conn).await?;
    Membership::drop_table(&mut *conn).await?;
    Ok(())
}
