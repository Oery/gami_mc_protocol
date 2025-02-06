use crate::{packets::Packet, registry::Dimension};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x07, server)]
pub struct Respawn {
    pub dimension: Dimension,
    pub difficulty: u8,
    pub gamemode: u8,
    pub level_type: String,
}
