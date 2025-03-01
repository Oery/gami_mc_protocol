use gami_macros::packet;

use crate::registry::entities::mobs::Mob;

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
