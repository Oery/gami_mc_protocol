use gami_macros::packet;

#[packet(0x18, server)]
pub struct EntityTeleport {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point")]
    pub x: i32,
    #[encoding("fixed_point")]
    pub y: i32,
    #[encoding("fixed_point")]
    pub z: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
