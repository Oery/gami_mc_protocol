use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x00, client)]
pub struct PingStart {}
