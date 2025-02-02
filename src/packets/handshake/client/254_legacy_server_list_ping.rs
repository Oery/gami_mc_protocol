use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0xfe, client)]
pub struct LegacyServerListPing {
    pub payload: u8,
}
