use gami_macros::packet;

use crate::registry::Difficulty;

#[packet(0x41, server)]
pub struct ServerDifficulty {
    pub difficulty: Difficulty,
}
