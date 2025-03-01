use gami_macros::packet;

use crate::registry::Vec3;

#[packet(0x0A, server)]
pub struct Bed {
    #[encoding("varint")]
    pub entity_id: i32,
    pub location: Vec3,
}
