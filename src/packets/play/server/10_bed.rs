use crate::packets::Packet;
use crate::registry::Vec3;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x0A, server)]
pub struct Bed {
    #[encoding("varint")]
    pub entity_id: i32,
    pub location: Vec3,
}
