use gami_macros::{Deserialize, Serialize};

use crate::serialization::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[encoding("i8")]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}
