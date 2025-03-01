use gami_macros::packet;

use crate::registry::Vec3;

#[packet(0x05, server)]
pub struct SpawnPosition {
    pub position: Vec3,
}
