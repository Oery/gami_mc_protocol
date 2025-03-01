use gami_macros::packet;

#[packet(0x05, client)]
pub struct Look {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
