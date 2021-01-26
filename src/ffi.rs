#![allow(non_camel_case_types)]

use super::poi::OsmCount;
use libc::c_ulong;
use std::convert::From;

/// The C representation type of OsmCount.
#[repr(C)]
pub struct OSMPOI_OsmCount {
    pub node: c_ulong,
    pub way: c_ulong,
    pub relation: c_ulong,
}

impl From<OsmCount> for OSMPOI_OsmCount {
    fn from(item: OsmCount) -> Self {
        let OsmCount {
            node,
            way,
            relation,
        } = item;
        OSMPOI_OsmCount {
            node,
            way,
            relation,
        }
    }
}

impl Into<OsmCount> for OSMPOI_OsmCount {
    fn into(self) -> OsmCount {
        let Self {
            node,
            way,
            relation,
        } = self;
        OsmCount {
            node,
            way,
            relation,
        }
    }
}
