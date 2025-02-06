use gami_macros::{packet, Deserialize, Serialize};

use crate::packets::Packet;
use crate::registry::{Difficulty, Dimension};

#[packet(0x07, server)]
pub struct Respawn {
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub gamemode: u8,
    pub level_type: String,
}
