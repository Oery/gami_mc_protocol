use gami_macros::packet;
use uuid::Uuid;

#[packet(0x0C, server)]
pub struct SpawnPlayer {
    #[encoding("varint")]
    pub entity_id: i32,
    pub player_uuid: Uuid,
    #[encoding("fixed_point")]
    pub x: i32,
    #[encoding("fixed_point")]
    pub y: i32,
    #[encoding("fixed_point")]
    pub z: i32,

    #[encoding("fixed_point")]
    pub yaw: f32,
    #[encoding("fixed_point")]
    pub pitch: f32,

    pub current_item: i16,
    pub metadatas: Vec<Metadata>,
}
