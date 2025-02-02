use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x1A, server)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}
