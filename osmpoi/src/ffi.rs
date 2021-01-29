use super::{count, dump, parse_relations, parse_ways, query_csv, refine, OsmCount};
use std::ffi::CStr;

/// The C representation type of OsmCount.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct OSMPOI_OsmCount {
    pub node: i64,
    pub way: i64,
    pub relation: i64,
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_count(path: *const i8) -> OSMPOI_OsmCount {
    match CStr::from_ptr(path).to_str() {
        Ok(path) => match count(path) {
            Ok(osm_count) => {
                let OsmCount {
                    node,
                    way,
                    relation,
                } = osm_count;
                OSMPOI_OsmCount {
                    node,
                    way,
                    relation,
                }
            }
            Err(_) => OSMPOI_OsmCount {
                node: -2,
                way: -2,
                relation: -2,
            },
        },
        Err(_) => OSMPOI_OsmCount {
            node: -1,
            way: -1,
            relation: -1,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_dump(pbf_path: *const i8, db_path: *const i8) -> i32 {
    match (
        CStr::from_ptr(pbf_path).to_str(),
        CStr::from_ptr(db_path).to_str(),
    ) {
        (Ok(pbf_path), Ok(db_path)) => match dump(pbf_path, db_path) {
            Ok(_) => 0,
            Err(_) => -2,
        },
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_parse_ways(dataset_path: *const i8) -> i32 {
    match CStr::from_ptr(dataset_path).to_str() {
        Ok(dataset_path) => match parse_ways(dataset_path) {
            Ok(_) => 0,
            Err(_) => -2,
        },
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_parse_relations(dataset_path: *const i8) -> i32 {
    match CStr::from_ptr(dataset_path).to_str() {
        Ok(dataset_path) => match parse_relations(dataset_path) {
            Ok(_) => 0,
            Err(_) => -2,
        },
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_refine(dataset_path: *const i8) -> i32 {
    match CStr::from_ptr(dataset_path).to_str() {
        Ok(dataset_path) => match refine(dataset_path) {
            Ok(_) => 0,
            Err(_) => -2,
        },
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_query_csv(
    input_path: *const i8,
    output_path: *const i8,
    dataset_path: *const i8,
    distance: f32,
    strict: bool,
) -> i32 {
    match (
        CStr::from_ptr(input_path).to_str(),
        CStr::from_ptr(output_path).to_str(),
        CStr::from_ptr(dataset_path).to_str(),
    ) {
        (Ok(input_path), Ok(output_path), Ok(dataset_path)) => {
            match query_csv(input_path, output_path, dataset_path, distance, strict) {
                Ok(_) => 0,
                Err(_) => -2,
            }
        }
        _ => -1,
    }
}
