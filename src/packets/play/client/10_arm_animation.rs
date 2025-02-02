use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x0A, client)]
pub struct ArmAnimation {}
