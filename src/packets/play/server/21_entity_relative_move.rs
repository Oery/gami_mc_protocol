use gami_macros::packet;

#[packet(0x15, server)]
pub struct EntityRelativeMove {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point_i8")]
    pub d_x: f32,
    #[encoding("fixed_point_i8")]
    pub d_y: f32,
    #[encoding("fixed_point_i8")]
    pub d_z: f32,
    pub on_ground: bool,
}
