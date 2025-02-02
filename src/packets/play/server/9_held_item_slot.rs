use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x09, server)]
pub struct HeldItemSlot {
    pub slot: i8,
}
