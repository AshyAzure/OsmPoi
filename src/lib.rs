pub mod csv;
pub mod ffi;
pub mod poi;

pub use ffi::*;

fn dis_to_deg(dis: f32) -> i32 {
    (dis / 6_371_000. / std::f32::consts::PI * 180.) as i32
}
