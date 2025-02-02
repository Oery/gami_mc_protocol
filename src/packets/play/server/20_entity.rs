use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use macros::{packet, Deserialize, Serialize};

#[packet(0x14, server)]
pub struct Entity {
    #[encoding("varint")]
    pub entity_id: i32,
}
