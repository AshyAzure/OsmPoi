use itertools::Itertools;
use osmpbfreader::{OsmId, OsmObj, OsmPbfReader};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::{Arc, RwLock};

pub fn main() {
    let path = get_path();
    let file = File::open(&path).expect(&format!("Fail to open file {}", path));
    let mut reader = OsmPbfReader::new(file);
    let required_info = extract_required(&mut reader);
    dump_tags(&format!("{}.tag.json", path), &required_info.tags);
    let positions = aggregate_position(required_info);
    dump_distances(&format!("{}.dist.csv", path), positions);
}

/// Calculate distances in a position map and dump it to a file.
fn dump_distances(path: &str, positions: PositionMap) {
    let mut wtr = csv::Writer::from_path(path).expect("Fail to create csv write");
    wtr.write_record(&["id1", "id2", "distance"])
        .expect("Fail to write header");
    for pair in positions.iter().combinations(2) {
        let (id1, pos1) = pair.get(0).expect("Fail to get 1st position");
        let (id2, pos2) = pair.get(1).expect("Fail to get 2nd position");
        let distance = calculate_distance(pos1, pos2);
        wtr.write_record(&[id1.to_string(), id2.to_string(), distance.to_string()])
            .expect("Fail to write csv row");
    }
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

/// Dump relation tags to a json file.
fn dump_tags(path: &str, tm: &TagMap) {
    let f = File::options()
        .write(true)
        .append(true)
        .open(path)
        .expect("Fail to open tag file");
    serde_json::to_writer_pretty(f, tm).expect("Fail to dump json to file");
}

/// Get PBF file path from CLI options.
fn get_path() -> String {
    let args: Vec<_> = std::env::args().collect();
    args.get(1).cloned().expect("usage: osmpoi <pbf-path>")
}

/// Aggregate relation position from other elements.
fn aggregate_position(required_info: RequiredInfo) -> HashMap<i64, Position> {
    let RequiredInfo {
        positions,
        relations,
        membership,
        ..
    } = required_info;
    let positions = Arc::new(RwLock::new(positions));
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
    positions
}

/// Calculate position using dynamic programming.
fn dp_position(id: &i64, positions: Arc<RwLock<PositionMap>>, membership: &MemberMap) -> Position {
    if let Some(pos) = positions.read().expect("Fail to read positions").get(id) {
        return *pos;
    }
    let mut mem_pos = vec![];
    for mid in membership.get(id).expect("Should have members") {
        mem_pos.push(dp_position(mid, positions.clone(), membership));
    }
    let pos = average_position(mem_pos);
    positions
        .write()
        .expect("Fail to write positions")
        .insert(*id, pos);
    pos
}

/// Calculate position from sub elements.
fn average_position(ps: Vec<Position>) -> Position {
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

/// Extract required info into memory.
fn extract_required(reader: &mut OsmPbfReader<File>) -> RequiredInfo {
    // record required information
    let mut required_nodes = HashSet::new();
    let mut required_ways = HashSet::new();
    let mut required_relations = HashSet::new();
    let mut pm = PositionMap::new();
    let mut mm = MemberMap::new();
    let mut tm = TagMap::new();
    // relation iter
    reader.rewind().expect("Fail to rewind");
    for obj in reader.par_iter().map(|o| o.expect("Fail to read par")) {
        if let OsmObj::Relation(r) = obj {
            // members
            for re in r.refs {
                let inner_id = re.member.inner_id();
                match re.member {
                    OsmId::Node(_) => {
                        required_nodes.insert(inner_id);
                    }
                    OsmId::Way(_) => {
                        required_ways.insert(inner_id);
                    }
                    _ => {}
                }
                required_relations.insert(r.id.0);
                mm.entry(r.id.0)
                    .and_modify(|v| v.push(inner_id))
                    .or_insert(vec![inner_id]);
            }
            // tags
            let mut tags = HashMap::new();
            for (tag_k, tag_v) in r.tags.iter() {
                tags.insert(tag_k.to_string(), tag_v.to_string());
            }
            tm.insert(r.id.0, tags);
        }
    }
    // way iter
    reader.rewind().expect("Fail to rewind");
    for obj in reader.par_iter().map(|o| o.expect("Fail to read par")) {
        if let OsmObj::Way(w) = obj {
            if required_ways.contains(&w.id.0) {
                // node members
                for node in w.nodes {
                    let inner_id = node.0;
                    required_nodes.insert(inner_id);
                    mm.entry(w.id.0)
                        .and_modify(|v| v.push(inner_id))
                        .or_insert(vec![inner_id]);
                }
            }
        }
    }
    // node iter
    reader.rewind().expect("Fail to rewind");
    for obj in reader.par_iter().map(|o| o.expect("Fail to read par")) {
        if let OsmObj::Node(n) = obj {
            if required_nodes.contains(&n.id.0) {
                // node position
                pm.insert(
                    n.id.0,
                    Position {
                        lat: n.lat(),
                        lon: n.lon(),
                        weight: 1.,
                    },
                );
            }
        }
    }
    RequiredInfo {
        positions: pm,
        membership: mm,
        relations: required_relations,
        tags: tm,
    }
}

type MemberMap = HashMap<i64, Vec<i64>>;

struct RequiredInfo {
    pub positions: PositionMap,
    pub relations: HashSet<i64>,
    pub membership: MemberMap,
    pub tags: TagMap,
}

type PositionMap = HashMap<i64, Position>;

#[derive(Clone, Copy, Debug)]
struct Position {
    pub lat: f64,
    pub lon: f64,
    pub weight: f64,
}

type TagMap = HashMap<i64, HashMap<String, String>>;
