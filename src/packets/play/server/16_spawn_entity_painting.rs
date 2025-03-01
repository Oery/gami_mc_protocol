use gami_macros::packet;

use crate::registry::Vec3;

#[packet(0x10, server)]
pub struct SpawnEntityPainting {
    #[encoding("varint")]
    pub entity_id: i32,
    pub title: String,
    pub location: Vec3,
    pub direction: u8,
}
