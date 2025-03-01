use crate::packets::Packet;
use crate::registry::entities::mobs::Mob;
use crate::serialization::{
    deserialize_fixed_point_i16, deserialize_fixed_point_i32, deserialize_metadatas_vec,
    deserialize_varint, serialize_fixed_point_i16, serialize_fixed_point_i32,
    serialize_metadatas_vec, serialize_varint, Metadata,
};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x0F, server)]
pub struct SpawnMob {
    #[encoding("varint")]
    pub entity_id: i32,
    pub kind: Mob,
    #[encoding("fixed_point_i32")]
    pub x: i32,
    #[encoding("fixed_point_i32")]
    pub y: i32,
    #[encoding("fixed_point_i32")]
    pub z: i32,
    pub yaw: u8,
    pub pitch: u8,
    pub head_pitch: u8,
    #[encoding("fixed_point_i16")]
    pub velocity_x: i16,
    #[encoding("fixed_point_i16")]
    pub velocity_y: i16,
    #[encoding("fixed_point_i16")]
    pub velocity_z: i16,
    #[encoding("metadatas")]
    pub metadatas: Vec<Metadata>,
}
