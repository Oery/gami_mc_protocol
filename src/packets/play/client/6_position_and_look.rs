use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x06, client)]
pub struct PositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
