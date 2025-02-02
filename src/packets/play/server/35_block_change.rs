use crate::packets::Packet;
use crate::registry::Vec3;
use macros::{packet, Deserialize, Serialize};

#[packet(0x23, server)]
pub struct BlockChange {
    pub location: Vec3,
    pub block_id: u16,
}
