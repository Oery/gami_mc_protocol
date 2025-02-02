use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x01, client)]
pub struct Ping {
    pub time: i64,
}
