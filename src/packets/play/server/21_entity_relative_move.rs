use gami_macros::packet;

#[packet(0x15, server)]
pub struct EntityRelativeMove {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point")]
    pub d_x: f32,
    #[encoding("fixed_point")]
    pub d_y: f32,
    #[encoding("fixed_point")]
    pub d_z: f32,
    pub on_ground: bool,
}
