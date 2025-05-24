use crate::serialization::{Deserialize, Serialize};
use derive_more::Display;
use gami_macros::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy, Display)]
#[encoding("u8")]
pub enum TextColor {
    Black = 0,
    DarkBlue = 1,
    DarkGreen = 2,
    DarkAqua = 3,
    DarkRed = 4,
    DarkPurple = 5,
    Gold = 6,
    Gray = 7,
    DarkGray = 8,
    Blue = 9,
    Green = 10,
    Aqua = 11,
    Red = 12,
    LightPurple = 13,
    Yellow = 14,
    #[default]
    White = 15,
}

impl From<u8> for TextColor {
    fn from(value: u8) -> Self {
        match value {
            0 => TextColor::Black,
            1 => TextColor::DarkBlue,
            2 => TextColor::DarkGreen,
            3 => TextColor::DarkAqua,
            4 => TextColor::DarkRed,
            5 => TextColor::DarkPurple,
            6 => TextColor::Gold,
            7 => TextColor::Gray,
            8 => TextColor::DarkGray,
            9 => TextColor::Blue,
            10 => TextColor::Green,
            11 => TextColor::Aqua,
            12 => TextColor::Red,
            13 => TextColor::LightPurple,
            14 => TextColor::Yellow,
            15 => TextColor::White,
            _ => TextColor::White,
        }
    }
}
