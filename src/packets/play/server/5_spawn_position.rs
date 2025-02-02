use crate::packets::Packet;
use crate::registry::Vec3;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x05, server)]
pub struct SpawnPosition {
    pub position: Vec3,
}
