use crate::orm::*;
use ormlite::sqlite::SqliteConnection;
use ormlite::Model;
use osmpbfreader::{OsmObj, OsmPbfReader};
use snafu::{prelude::*, Whatever};
use std::fs::File;

pub async fn extract_required(
    reader: &mut OsmPbfReader<File>,
    conn: &mut SqliteConnection,
) -> Result<(), Whatever> {
    Tag::create_table(conn).await?;
    Membership::create_table(conn).await?;
    Relation::create_table(conn).await?;
    Position::create_table(conn).await?;
    // iterate relations
    reader
        .rewind()
        .whatever_context("Fail to rewind reader for extracting relations")?;
    for obj in reader.par_iter() {
        // only handle relations
        if let Ok(OsmObj::Relation(r)) = obj {
            // extract relation id
            Relation { id: r.id.0 }
                .insert(&mut *conn)
                .await
                .whatever_context("Fail to insert relation id")?;
            // extract member id
            for m in r.refs {
                Membership {
                    id: r.id.0,
                    mid: m.member.inner_id(),
                }
                .insert(&mut *conn)
                .await
                .whatever_context("Fail to insert member of relation")?;
            }
            // extract tags
            for (tk, tv) in r.tags.iter() {
                let tag = Tag {
                    id: r.id.0,
                    key: tk.to_string(),
                    value: tv.to_string(),
                };
                tag.insert(&mut *conn)
                    .await
                    .whatever_context("Fail to insert tag")?;
            }
        }
    }
    // iterate ways
    reader
        .rewind()
        .whatever_context("Fail to rewind reader for extracting ways")?;
    for obj in reader.par_iter() {
        // only handle ways
        if let Ok(OsmObj::Way(w)) = obj {
            // only handle member of relation
            if let Ok(_) = Membership::select()
                .where_("mid = ?")
                .bind(w.id.0)
                .fetch_one(&mut *conn)
                .await
            {
                // extract member node id
                for n in w.nodes {
                    Membership {
                        id: w.id.0,
                        mid: n.0,
                    }
                    .insert(&mut *conn)
                    .await
                    .whatever_context("Fail to insert member of way")?;
                }
            }
        }
    }
    // iterate over nodes
    reader
        .rewind()
        .whatever_context("Fail to rewind reader for extracting nodes")?;
    for obj in reader.par_iter() {
        // only handle nodes
        if let Ok(OsmObj::Node(n)) = obj {
            // only handle member of relation
            if let Ok(_) = Membership::select()
                .where_("mid = ?")
                .bind(n.id.0)
                .fetch_one(&mut *conn)
                .await
            {
                // extract position
                Position {
                    id: n.id.0,
                    lat: n.lat(),
                    lon: n.lon(),
                    weight: 1.,
                }
                .insert(&mut *conn)
                .await
                .whatever_context("Fail to insert position of node")?;
            }
        }
    }
    Ok(())
}
