use super::{count, query_csv, OsmCount};
use std::ffi::CStr;
use std::os::raw::c_char;

/// The C representation type of OsmCount.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct OSMPOI_OsmCount {
    pub node: i64,
    pub way: i64,
    pub relation: i64,
}

#[no_mangle]
pub unsafe extern "C" fn osmpoi_count(path: *const c_char) -> OSMPOI_OsmCount {
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
pub unsafe extern "C" fn osmpoi_query_csv(
    input_path: *const c_char,
    output_path: *const c_char,
    dataset_path: *const c_char,
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
