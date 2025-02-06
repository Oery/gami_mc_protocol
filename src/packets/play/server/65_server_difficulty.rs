use gami_macros::{packet, Deserialize, Serialize};

use crate::packets::Packet;
use crate::registry::Difficulty;

#[packet(0x41, server)]
pub struct ServerDifficulty {
    pub difficulty: Difficulty,
}
