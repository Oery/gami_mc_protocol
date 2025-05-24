use derive_more::Display;
use gami_macros::{Deserialize, Serialize};

use crate::serialization::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Default, Display)]
#[encoding("i8")]
pub enum Dimension {
    Nether = -1,
    #[default]
    Overworld = 0,
    End = 1,
}
