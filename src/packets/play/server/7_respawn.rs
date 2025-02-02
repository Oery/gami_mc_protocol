use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x07, server)]
pub struct Respawn {
    pub dimension: i32,
    pub difficulty: u8,
    pub gamemode: u8,
    pub level_type: String,
}
