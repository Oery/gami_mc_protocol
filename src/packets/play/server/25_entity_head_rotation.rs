use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x19, server)]
pub struct EntityHeadRotation {
    #[encoding("varint")]
    pub entity_id: i32,
    pub head_yaw: i8,
}
