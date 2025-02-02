use crate::serialization::{Deserialize, Serialize};
use gami_macros::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[encoding("u8")]
pub enum ChatModes {
    Full = 0,
    CommandsOnly = 1,
    Hidden = 2,
}
