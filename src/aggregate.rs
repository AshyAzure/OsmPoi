use crate::orm::{Membership, Position};
use ormlite::sqlite::SqliteConnection;
use ormlite::Model;
use osmpbfreader::OsmPbfReader;
use snafu::{prelude::*, Whatever};
use std::fs::File;
use tokio::sync::{mpsc, oneshot};

/// Aggregate relation position from other elements.
async fn aggregate_position(
    reader: &mut OsmPbfReader<File>,
    conn: &SqliteConnection,
    n_elements: u64,
    rx: mpsc::Receiver<oneshot::Sender<String>>,
) -> Result<(), Whatever> {
    let mut count = 0;
    loop {
        if let Ok(r) = Membership::select()
            .where_("is_relation = TRUE")
            .order_asc("id")
            .offset(count)
            .fetch_one(&mut *conn)
            .await
        {
        } else {
            break;
        }
    }
    // TODO: query and iterate
    for id in relations.iter() {
        let positions = positions.clone();
        dp_position(id, positions, &membership);
    }
    let mut positions = Arc::try_unwrap(positions)
        .expect("Fail to unwrap Arc")
        .into_inner()
        .expect("Fail to get inner positions");
    // retain relation positions only
    positions.retain(|&k, _| relations.contains(&k));
    Ok(())
}

/// Calculate position using dynamic programming.
fn dp_position(id: &i64, positions: Arc<RwLock<PositionMap>>, membership: &MemberMap) -> Position {
    if let Some(pos) = positions.read().expect("Fail to read positions").get(id) {
        return *pos;
    }
    let mut mem_pos = vec![];
    if let Some(mids) = membership.get(id) {
        for mid in mids {
            mem_pos.push(dp_position(mid, positions.clone(), membership));
        }
    }
    let pos = average_positions(mem_pos);
    positions
        .write()
        .expect("Fail to write positions")
        .insert(*id, pos);
    pos
}

/// Calculate position using dynamic programming.
fn dp_position_new(id: i64, conn: &SqliteConnection) -> Position {
    if let Ok(Some(position)) = positions.get(&id) {
        position
    } else {
        let mut member_positions = vec![];
        if let Ok(Some(member_ids)) = membership.get(&id) {
            for member_id in member_ids {
                member_positions.push(dp_position_new(member_id, positions, membership));
            }
        }
        let position = average_positions(member_positions);
        position
    }
}

/// Calculate position from sub elements.
fn average_positions(ps: Vec<Position>) -> Position {
    let mut lat = 0.;
    let mut lon = 0.;
    let mut weight = 0.;
    for p in ps {
        lat += p.lat;
        lon += p.lon;
        weight += p.weight;
    }
    lat /= weight;
    lon /= weight;
    Position { lat, lon, weight }
}
