use gami_macros::packet;

use crate::registry::Vec3;

#[packet(0x23, server)]
pub struct BlockChange {
    pub location: Vec3,
    #[encoding("varint")]
    pub block_id: i32,
}
