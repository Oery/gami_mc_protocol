use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x01, client)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
