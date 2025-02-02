use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x00, client)]
pub struct PingStart {}
