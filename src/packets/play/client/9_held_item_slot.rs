use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x09, client)]
pub struct HeldItemSlot {
    pub slot: i16,
}
