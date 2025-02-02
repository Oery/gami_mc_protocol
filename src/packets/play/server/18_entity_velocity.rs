use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x12, server)]
pub struct EntityVelocity {
    #[encoding("varint")]
    pub entity_id: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
