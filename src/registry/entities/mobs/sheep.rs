use std::fmt;

use gami_macros::entity;

#[entity(LivingEntity, Ageable)]
pub struct Sheep {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub color: SheepColor,
    pub is_sheared: bool,
}

impl Sheep {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(16, value) = metadata {
                self.color = SheepColor::from(*value & 0x0F);
                self.is_sheared = (*value & 0x10) != 0;
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum SheepColor {
    #[default]
    White = 0,
    Orange = 1,
    Magenta = 2,
    LightBlue = 3,
    Yellow = 4,
    Lime = 5,
    Pink = 6,
    Gray = 7,
    LightGray = 8,
    Cyan = 9,
    Purple = 10,
    Blue = 11,
    Brown = 12,
    Green = 13,
    Red = 14,
    Black = 15,
}

impl From<u8> for SheepColor {
    fn from(value: u8) -> Self {
        match value {
            0 => SheepColor::White,
            1 => SheepColor::Orange,
            2 => SheepColor::Magenta,
            3 => SheepColor::LightBlue,
            4 => SheepColor::Yellow,
            5 => SheepColor::Lime,
            6 => SheepColor::Pink,
            7 => SheepColor::Gray,
            8 => SheepColor::LightGray,
            9 => SheepColor::Cyan,
            10 => SheepColor::Purple,
            11 => SheepColor::Blue,
            12 => SheepColor::Brown,
            13 => SheepColor::Green,
            14 => SheepColor::Red,
            15 => SheepColor::Black,
            _ => SheepColor::White,
        }
    }
}

impl fmt::Display for SheepColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
