#![allow(non_camel_case_types)]

use super::poi::OsmCount;
use ffi_convert::{AsRust, CDrop, CReprOf};

/// The C representation type of OsmCount.
#[repr(C)]
#[derive(CReprOf, AsRust, CDrop)]
#[target_type(OsmCount)]
pub struct OSMPOI_OsmCount {
    pub node: u64,
    pub way: u64,
    pub relation: u64,
}
