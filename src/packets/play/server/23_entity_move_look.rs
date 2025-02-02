use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x17, server)]
pub struct EntityMoveLook {
    #[encoding("varint")]
    pub entity_id: i32,
    pub d_x: i8,
    pub d_y: i8,
    pub d_z: i8,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
