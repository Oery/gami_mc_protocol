use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x01, server)]
pub struct Ping {
    pub time: i64,
}
