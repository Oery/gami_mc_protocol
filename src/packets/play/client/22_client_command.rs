use crate::packets::Packet;
use crate::registry::ClientCommandActions;
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x16, client)]
pub struct ClientCommand {
    pub action: ClientCommandActions,
}

impl ClientCommand {
    pub fn respawn() -> Self {
        Self {
            action: ClientCommandActions::PerformRespawn,
        }
    }
}
