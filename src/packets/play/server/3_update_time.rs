use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x03, server)]
pub struct UpdateTime {
    pub age: i64,
    pub time: i64,
}
