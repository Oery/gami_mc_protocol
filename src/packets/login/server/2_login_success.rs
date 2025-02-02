use crate::packets::Packet;
use macros::{packet, Deserialize, Serialize};

#[packet(0x02, server)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
