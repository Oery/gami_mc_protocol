use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x01, client)]
pub struct Chat {
    pub message: String,
}

impl Chat {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}
