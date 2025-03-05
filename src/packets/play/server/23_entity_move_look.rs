use gami_macros::packet;

#[packet(0x17, server)]
pub struct EntityMoveLook {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point")]
    pub d_x: f32,
    #[encoding("fixed_point")]
    pub d_y: f32,
    #[encoding("fixed_point")]
    pub d_z: f32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
