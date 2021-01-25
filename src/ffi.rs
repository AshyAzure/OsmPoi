#![allow(non_camel_case_types)]

use super::poi::OsmCount;
use ffi_convert::{AsRust, CDrop, CReprOf};
use libc::c_ulong;

/// The C representation type of OsmCount.
#[repr(C)]
#[derive(CReprOf, AsRust, CDrop)]
#[target_type(OsmCount)]
pub struct OSMPOI_OsmCount {
    pub node: c_ulong,
    pub way: c_ulong,
    pub relation: c_ulong,
}
