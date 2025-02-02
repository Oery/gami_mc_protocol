use crate::packets::Packet;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x1B, server)]
pub struct AttachEntity {
    pub entity_id: i32,
    pub vehicle_id: i32,
    pub leash: bool,
}
