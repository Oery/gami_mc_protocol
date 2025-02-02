use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x0A, client)]
pub struct ArmAnimation {}
