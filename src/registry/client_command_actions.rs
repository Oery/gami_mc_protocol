use crate::serialization::{Deserialize, Serialize};
use macros::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[encoding("u8")]
pub enum ClientCommandActions {
    PerformRespawn = 0,
    RequestStats = 1,
    TakingInventoryAchievement = 2,
}
