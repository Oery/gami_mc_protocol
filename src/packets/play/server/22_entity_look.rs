use gami_macros::packet;

#[packet(0x16, server)]
pub struct EntityLook {
    #[encoding("varint")]
    pub entity_id: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
