use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x0D, client)]
pub struct CloseWindow {
    pub window_id: u8,
}
