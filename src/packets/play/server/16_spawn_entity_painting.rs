use crate::packets::Packet;
use crate::registry::Vec3;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x10, server)]
pub struct SpawnEntityPainting {
    #[encoding("varint")]
    pub entity_id: i32,
    pub title: String,
    pub location: Vec3,
    pub direction: u8,
}
