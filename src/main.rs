use im_rc::HashSet;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use osmpbf::{BlobReader, Element};
use serde::{Deserialize, Serialize};
use sled::{
    transaction::{ConflictableTransactionError, UnabortableTransactionError},
    Db,
};
use std::{borrow::Cow, cell::RefCell, rc::Rc};
use typed_sled::{transaction::Transactional, Tree};

lazy_static! {
    /// The gloabl multi progress bar.
    static ref MP: MultiProgress = MultiProgress::new();
}

pub fn main() {
    // process args
    let args: Vec<_> = std::env::args().collect();
    let path = args
        .get(1)
        .unwrap_or_else(|| panic!("Fail to get path from command line args"));
    // initialize database (relation is stored in chunks)
    let db = sled::Config::new()
        .path(format!("{path}.sled"))
        .temporary(true)
        .open()
        .unwrap();
    // count blobs
    let n_blob = count_blob(path);
    // dump required data
    let n_relation = dump_info(path, n_blob, &db);
    // aggregate_position
    aggregate_position(n_relation, &db);
    // calculate distances
    calculate_distance(path, n_relation, &db);
}

fn count_blob(path: &str) -> u64 {
    let pb = add_spinner("count blobs");
    let rdr = BlobReader::from_path(path).unwrap();
    let mut n_blob = 0;
    // iterate and add
    for blob_res in rdr {
        if let Ok(_blob) = blob_res {
            n_blob += 1;
            pb.inc(1);
        }
    }
    pb.finish();
    n_blob
}

fn dump_info(path: &str, n_blob: u64, db: &Db) -> u64 {
    let n_relation = dump_relation(path, n_blob, db);
    dump_way(path, n_blob, db);
    dump_node(path, n_blob, db);
    n_relation
}

fn dump_relation(path: &str, n_blob: u64, db: &Db) -> u64 {
    let member_tree: Tree<i64, Vec<i64>> = Tree::open(&db, "member");
    let relation_tree: Tree<u64, Vec<i64>> = Tree::open(&db, "relation");
    let required_tree: Tree<i64, ()> = Tree::open(&db, "required");
    let pb = add_progressbar("dump relation", n_blob);
    let rdr = BlobReader::from_path(path).unwrap();
    let wtr = csv::Writer::from_path(format!("{path}.tag.csv")).unwrap();
    let wtr_rc = Rc::new(RefCell::new(wtr));
    let chunk_rc = Rc::new(RefCell::new(RelationChunk {
        idx: 0,
        relations: vec![],
    }));
    let mut n_relation = 0;
    for blob_res in rdr {
        if let Ok(blob) = blob_res {
            pb.inc(1);
            // only handle data block
            if let Ok(block) = blob.to_primitiveblock() {
                let wtr_rc = wtr_rc.clone();
                n_relation += (&required_tree, &member_tree, &relation_tree)
                    .transaction(|(rqt, mt, rt)| {
                        let mut blob_n_relation = 0;
                        for el in block.elements() {
                            if let Element::Relation(r) = el {
                                let id = r.id();
                                pb.set_message(format!("relation {id}"));
                                mt.insert(&id, &r.members().map(|m| m.member_id).collect())?;
                                for m in r.members() {
                                    rqt.insert(&m.member_id, &())?;
                                }
                                blob_n_relation += 1;
                                let mut chunk = chunk_rc.borrow_mut();
                                chunk.relations.push(id);
                                if chunk.relations.len() >= 1000 {
                                    rt.insert(&chunk.idx, &chunk.relations)?;
                                    chunk.relations.clear();
                                    chunk.idx += 1;
                                }
                                for (key, val) in r.tags() {
                                    let mut wtr = wtr_rc.borrow_mut();
                                    wtr.serialize(TagRow { id, key, val }).unwrap();
                                }
                            } else {
                                break;
                            }
                        }
                        Ok::<_, ConflictableTransactionError<UnabortableTransactionError>>(
                            blob_n_relation,
                        )
                    })
                    .unwrap();
            }
        }
    }
    pb.finish();
    n_relation
}

fn dump_way(path: &str, n_blob: u64, db: &Db) {
    let member_tree: Tree<i64, Vec<i64>> = Tree::open(&db, "member");
    let required_tree: Tree<i64, ()> = Tree::open(&db, "required");
    let pb = add_progressbar("dump way", n_blob);
    let rdr = BlobReader::from_path(path).unwrap();
    for blob_res in rdr {
        if let Ok(blob) = blob_res {
            pb.inc(1);
            // only handle data block
            if let Ok(block) = blob.to_primitiveblock() {
                (&required_tree, &member_tree)
                    .transaction(|(rqt, mt)| {
                        for el in block.elements() {
                            if let Element::Way(w) = el {
                                let id = w.id();
                                if let Ok(Some(_)) = rqt.get(&id) {
                                    pb.set_message(format!("way {id}"));
                                    mt.insert(&id, &w.raw_refs().iter().map(|x| *x).collect())?;
                                    for nid in w.raw_refs() {
                                        rqt.insert(nid, &())?;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Ok::<_, ConflictableTransactionError<UnabortableTransactionError>>(())
                    })
                    .unwrap();
            }
        }
    }
    pb.finish();
}

fn dump_node(path: &str, n_blob: u64, db: &Db) {
    let position_tree: Tree<i64, Position> = Tree::open(&db, "position");
    let required_tree: Tree<i64, ()> = Tree::open(&db, "required");
    let pb = add_progressbar("dump node", n_blob);
    let rdr = BlobReader::from_path(path).unwrap();
    for blob_res in rdr {
        if let Ok(blob) = blob_res {
            pb.inc(1);
            // only handle data block
            if let Ok(block) = blob.to_primitiveblock() {
                (&position_tree, &required_tree)
                    .transaction(|(pt, rqt)| {
                        for el in block.elements() {
                            match el {
                                Element::Node(n) => {
                                    let id = n.id();
                                    if let Ok(Some(_)) = rqt.get(&id) {
                                        pb.set_message(format!("node {id}"));
                                        pt.insert(
                                            &id,
                                            &Position {
                                                lat: n.lat(),
                                                lon: n.lon(),
                                                weight: 1.,
                                            },
                                        )?;
                                    }
                                }
                                // dense node, position only
                                Element::DenseNode(dn) => {
                                    let id = dn.id;
                                    if let Ok(Some(_)) = rqt.get(&id) {
                                        pb.set_message(format!("node {id}"));
                                        pt.insert(
                                            &id,
                                            &Position {
                                                lat: dn.lat(),
                                                lon: dn.lon(),
                                                weight: 1.,
                                            },
                                        )?;
                                    }
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        Ok::<_, ConflictableTransactionError<UnabortableTransactionError>>(())
                    })
                    .unwrap();
            }
        }
    }
    pb.finish();
}

fn aggregate_position(n_relation: u64, db: &Db) {
    let member_tree: Tree<i64, Vec<i64>> = Tree::open(&db, "member");
    let relation_tree: Tree<u64, Vec<i64>> = Tree::open(&db, "relation");
    let position_tree: Tree<i64, Position> = Tree::open(&db, "position");
    let pb = add_progressbar("aggregate position", n_relation);
    for kv_res in relation_tree.iter() {
        if let Ok((_chunk_idx, rid_chunk)) = kv_res {
            for rid in rid_chunk {
                pb.inc(1);
                // Dynamic programming using the position tree.
                dp_position(rid, &pb, &position_tree, &member_tree, &HashSet::new());
            }
        }
    }
    pb.finish();
    fn dp_position(
        id: i64,
        pb: &ProgressBar,
        position_tree: &Tree<i64, Position>,
        member_tree: &Tree<i64, Vec<i64>>,
        set: &HashSet<i64>,
    ) -> Position {
        pb.set_message(format!("dp {id}"));
        if let Ok(Some(pos)) = position_tree.get(&id) {
            pos
        } else {
            let set = set.clone().union(HashSet::unit(id));
            let mut pos = Position {
                lat: 0.,
                lon: 0.,
                weight: 0.,
            };
            if let Ok(Some(mids)) = member_tree.get(&id) {
                for mid in mids {
                    if set.contains(&mid) {
                        continue;
                    }
                    let mpos = dp_position(mid, pb, position_tree, member_tree, &set);
                    pos.lat += mpos.lat * mpos.weight;
                    pos.lon += mpos.lon * mpos.weight;
                    pos.weight += mpos.weight;
                }
            }
            pos.lat /= pos.weight;
            pos.lon /= pos.weight;
            position_tree.insert(&id, &pos).unwrap();
            pos
        }
    }
}

fn calculate_distance(path: &str, n_relation: u64, db: &Db) {
    let relation_tree: Tree<u64, Vec<i64>> = Tree::open(&db, "relation");
    let position_tree: Tree<i64, Position> = Tree::open(&db, "position");
    let pb = add_progressbar("calculate distance", n_relation);
    let mut wtr = csv::Writer::from_path(format!("{path}.distance.csv")).unwrap();
    // outer, id1
    for outer_res in relation_tree.iter() {
        if let Ok((_, id1_chunk)) = outer_res {
            for id1 in id1_chunk {
                pb.inc(1);
                if let Ok(Some(pos1)) = position_tree.get(&id1) {
                    if pos1.lat.is_nan() || pos1.lon.is_nan() {
                        continue;
                    }
                    // inner, id2
                    'inner: for inner_res in relation_tree.iter() {
                        if let Ok((_, id2_chunk)) = inner_res {
                            for id2 in id2_chunk {
                                // use id ordering to ensure each pair calculated only once
                                if id1 >= id2 {
                                    continue 'inner;
                                }
                                // add only if both position exists
                                if let Ok(Some(pos2)) = position_tree.get(&id2) {
                                    if pos2.lat.is_nan() || pos2.lon.is_nan() {
                                        continue;
                                    }
                                    pb.set_message(format!("{id1} x {id2}"));
                                    let km = haversine(&pos1, &pos2);
                                    if !km.is_nan() {
                                        wtr.serialize(DistanceRow { id1, id2, km }).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pb.finish();
    fn haversine(pos1: &Position, pos2: &Position) -> f64 {
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
}

fn add_spinner(prefix: impl Into<Cow<'static, str>>) -> ProgressBar {
    let pb = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template("{prefix}, progress: {pos}/???").unwrap())
        .with_prefix(prefix);
    MP.add(pb.clone());
    pb
}

fn add_progressbar(prefix: impl Into<Cow<'static, str>>, len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len)
        .with_style(
            ProgressStyle::with_template("{prefix}, progress: {pos}/{len}, eta: {eta}, msg: {msg}")
                .unwrap(),
        )
        .with_prefix(prefix);
    MP.add(pb)
}

#[derive(Deserialize, Serialize)]
struct Position {
    lat: f64,
    lon: f64,
    weight: f64,
}

#[derive(Serialize)]
struct TagRow<'a> {
    id: i64,
    key: &'a str,
    val: &'a str,
}

struct RelationChunk {
    idx: u64,
    relations: Vec<i64>,
}

#[derive(Serialize)]
struct DistanceRow {
    id1: i64,
    id2: i64,
    km: f64,
}
