use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x40, server)]
pub struct KickDisconnect {
    pub reason: String,
}
