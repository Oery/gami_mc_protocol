use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x03, client)]
pub struct Flying {
    pub on_ground: bool,
}
