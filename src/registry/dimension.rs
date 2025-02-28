use gami_macros::{Deserialize, Serialize};

use crate::serialization::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Default)]
#[encoding("i8")]
pub enum Dimension {
    Nether = -1,
    #[default]
    Overworld = 0,
    End = 1,
}
