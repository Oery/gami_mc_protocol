use crate::packets::Packet;
use crate::registry::ChatModes;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x15, client)]
pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatModes,
    pub chat_colors: bool,
    pub skin_parts: u8,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 5,
            chat_mode: ChatModes::Full,
            chat_colors: true,
            skin_parts: 255,
        }
    }
}
