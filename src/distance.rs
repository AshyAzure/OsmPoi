use crate::orm::{Distance, Position, Relation};
use ormlite::sqlite::SqliteConnection;
use ormlite::Model;
use snafu::{prelude::*, Whatever};

/// Calculate distances in a position map and dump it to a file.
pub async fn dump_distances(conn: &mut SqliteConnection) -> Result<(), Whatever> {
    // init table
    Distance::create_table(&mut *conn).await?;
    // iter twicely over relations
    // loop outer
    let mut offset_outer = 0;
    'outer: loop {
        if let Ok(r1) = Relation::select()
            .offset(offset_outer)
            .fetch_one(&mut *conn)
            .await
        {
            // loop inner after offset of outer
            let mut offset_inner = offset_outer + 1;
            'inner: loop {
                if let Ok(r2) = Relation::select()
                    .offset(offset_inner)
                    .fetch_one(&mut *conn)
                    .await
                {
                    // get position and calculate distance
                    dump_pair(r1.id, r2.id, &mut *conn).await?;
                    // inner increase offset
                    offset_inner += 1;
                } else {
                    break 'inner;
                }
            }
            // outer increase offset
            offset_outer += 1;
        } else {
            break 'outer;
        }
    }
    Ok(())
}

/// Dump distance of a pair.
async fn dump_pair(id1: i64, id2: i64, conn: &mut SqliteConnection) -> Result<(), Whatever> {
    let pos1 = Position::select()
        .where_("id = ?")
        .bind(id1)
        .fetch_one(&mut *conn)
        .await
        .whatever_context("Fail to get position of relation")?;
    let pos2 = Position::select()
        .where_("id = ?")
        .bind(id1)
        .fetch_one(&mut *conn)
        .await
        .whatever_context("Fail to get position of relation")?;
    let distance = Distance {
        id1,
        id2,
        km: calculate_distance(&pos1, &pos2),
    };
    distance
        .insert(&mut *conn)
        .await
        .whatever_context("Fail to insert into distance table")?;
    Ok(())
}

/// Calculate distance between two positions in kilometers.
///
/// Stolen from
/// <https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/trigonometry.html>.
fn calculate_distance(pos1: &Position, pos2: &Position) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    let lat1_rad = pos1.lat.to_radians();
    let lat2_rad = pos2.lat.to_radians();
    let delta_latitude = (pos1.lat - pos2.lat).to_radians();
    let delta_longitude = (pos1.lon - pos2.lon).to_radians();
    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();
    let distance = earth_radius_kilometer * central_angle;
    distance
}
