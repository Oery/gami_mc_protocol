use gami_macros::{packet, Deserialize, Serialize};

use crate::packets::Packet;
use crate::registry::Vec3;
use crate::serialization::{deserialize_varint, serialize_varint};

#[packet(0x23, server)]
pub struct BlockChange {
    pub location: Vec3,
    #[encoding("varint")]
    pub block_id: i32,
}
