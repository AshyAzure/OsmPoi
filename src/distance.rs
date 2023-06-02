/// Calculate distances in a position map and dump it to a file.
fn dump_distances(conn: &SqliteConnection) {
    let mut wtr = csv::Writer::from_path(path).expect("Fail to create csv write");
    wtr.write_record(&["id1", "id2", "distance"])
        .expect("Fail to write header");
    for (k1, k2) in positions.keys().expect("Fail to get keys").combinations(2) {}
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
