use gami_macros::{packet, Deserialize, Serialize};

use crate::packets::Packet;
use crate::registry::{Difficulty, Dimension};

#[packet(0x01, server)]
pub struct JoinGame {
    pub entity_id: i32,
    pub game_mode: u8,
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub max_players: u8,
    pub level_type: String,
    pub reduced_debug_info: bool,
}
