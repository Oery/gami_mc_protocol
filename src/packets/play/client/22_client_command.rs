use gami_macros::packet;

use crate::registry::ClientCommandActions;

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
