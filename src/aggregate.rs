use crate::orm::{Membership, Position, Relation};
use async_recursion::async_recursion;
use ormlite::sqlite::SqliteConnection;
use ormlite::Model;
use snafu::{prelude::*, Whatever};

/// Aggregate relation position from other elements.
pub async fn aggregate_positions(conn: &mut SqliteConnection) -> Result<(), Whatever> {
    let mut offset = 0;
    // calculate with dynamic programming
    loop {
        if let Ok(r) = Relation::select()
            .offset(offset)
            .fetch_one(&mut *conn)
            .await
        {
            offset += 1;
            dp_position(r.id, conn).await?;
        } else {
            // no more relation
            break;
        }
    }
    Ok(())
}

/// Calculate position using dynamic programming.
#[async_recursion]
async fn dp_position(id: i64, conn: &mut SqliteConnection) -> Result<Position, Whatever> {
    // if already in position table, return;
    if let Ok(pos) = Position::select()
        .where_("id = ?")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
    {
        return Ok(pos);
    }
    // otherwise calculate it
    let mut pos = Position {
        id,
        lat: 0.,
        lon: 0.,
        weight: 0.,
    };
    // iterate over member and add it to pos
    let mut offset = 0;
    loop {
        if let Ok(r) = Membership::select()
            .where_("id = ?")
            .bind(id)
            .offset(offset)
            .fetch_one(&mut *conn)
            .await
        {
            offset += 1;
            // adding sub pos to current
            if let Ok(sub_pos) = dp_position(r.id, conn).await {
                pos.lat += sub_pos.lat * sub_pos.weight;
                pos.lon += sub_pos.lon * sub_pos.weight;
                pos.weight += sub_pos.weight;
            } else {
                println!("Fail to get member position")
            }
        } else {
            // no more member
            break;
        }
    }
    pos.lat /= pos.weight;
    pos.lon /= pos.weight;
    pos.insert(&mut *conn)
        .await
        .whatever_context("Fail to save aggregated position")?;
    Ok(pos)
}
