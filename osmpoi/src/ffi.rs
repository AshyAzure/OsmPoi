#![allow(non_camel_case_types)]

use super::poi::{OsmCount, OsmPbfReaderExt};
use osmpbfreader::OsmPbfReader;
use rusqlite::Transaction;
use std::ffi::CStr;
use std::fs::File;

/// The C representation type of OsmCount.
#[repr(C)]
pub struct OSMPOI_OsmCount {
    pub node: u64,
    pub way: u64,
    pub relation: u64,
}

type OsmPbfFileReader = OsmPbfReader<File>;

/// create an osmpbfreader
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn osmpoi_create_reader(path: *const i8) -> *mut OsmPbfFileReader {
    if let Ok(path) = CStr::from_ptr(path).to_str() {
        if let Ok(read) = File::open(path) {
            Box::into_raw(Box::new(OsmPbfReader::new(read)))
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// count the reader
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn osmpoi_count_reader(pbf: *mut OsmPbfFileReader) -> OSMPOI_OsmCount {
    if let Some(pbf) = pbf.as_mut() {
        match pbf.count() {
            Ok(count) => {
                let OsmCount {
                    node,
                    way,
                    relation,
                } = count;
                OSMPOI_OsmCount {
                    node,
                    way,
                    relation,
                }
            }
            Err(_) => OSMPOI_OsmCount {
                node: 0,
                way: 0,
                relation: 0,
            },
        }
    } else {
        OSMPOI_OsmCount {
            node: 0,
            way: 0,
            relation: 0,
        }
    }
}

/// dump the reader
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn osmpoi_dump_reader(
    pbf: *mut OsmPbfFileReader,
    tx: *mut Transaction,
) -> i32 {
    if let (Some(pbf), Some(tx)) = (pbf.as_mut(), tx.as_mut()) {
        match pbf.dump(tx) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        -2
    }
}

/// destroy the reader
/// # Safety
pub unsafe extern "C" fn osmpoi_destroy_reader(pbf: *mut OsmPbfFileReader) -> i32 {
    if !pbf.is_null() {
        let _ = Box::from_raw(pbf); // get box and drop
        return 0;
    }
    -1
}
