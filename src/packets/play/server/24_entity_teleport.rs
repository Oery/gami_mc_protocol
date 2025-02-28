use crate::packets::Packet;
use crate::serialization::{deserialize_fixed_point_i32, serialize_fixed_point_i32};
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x18, server)]
pub struct EntityTeleport {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point_i32")]
    pub x: i32,
    #[encoding("fixed_point_i32")]
    pub y: i32,
    #[encoding("fixed_point_i32")]
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
