use crate::serialization::{Deserialize, Serialize};
use gami_macros::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[encoding("u8")]
pub enum CollarColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Brown = 3,
    Blue = 4,
    Purple = 5,
    Cyan = 6,
    LightGray = 7,
    Gray = 8,
    Pink = 9,
    Lime = 10,
    Yellow = 11,
    LightBlue = 12,
    Magenta = 13,
    Orange = 14,
    #[default]
    White = 15,
}

impl From<u8> for CollarColor {
    fn from(value: u8) -> Self {
        match value {
            0 => CollarColor::Black,
            1 => CollarColor::Red,
            2 => CollarColor::Green,
            3 => CollarColor::Brown,
            4 => CollarColor::Blue,
            5 => CollarColor::Purple,
            6 => CollarColor::Cyan,
            7 => CollarColor::LightGray,
            8 => CollarColor::Gray,
            9 => CollarColor::Pink,
            10 => CollarColor::Lime,
            11 => CollarColor::Yellow,
            12 => CollarColor::LightBlue,
            13 => CollarColor::Magenta,
            14 => CollarColor::Orange,
            15 => CollarColor::White,
            _ => CollarColor::White,
        }
    }
}

impl std::fmt::Display for CollarColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
