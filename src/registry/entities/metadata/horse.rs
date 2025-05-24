#[derive(Debug, Default, PartialEq)]
pub enum HorseKind {
    #[default]
    Horse = 0,
    Donkey = 1,
    Mule = 2,
    Zombie = 3,
    Skeleton = 4,
}

impl From<u8> for HorseKind {
    fn from(value: u8) -> Self {
        match value {
            0 => HorseKind::Horse,
            1 => HorseKind::Donkey,
            2 => HorseKind::Mule,
            3 => HorseKind::Zombie,
            4 => HorseKind::Skeleton,
            _ => HorseKind::Horse,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum HorseColor {
    #[default]
    White = 0,
    Creamy = 1,
    Chestnut = 2,
    Brown = 3,
    Black = 4,
    Gray = 5,
    DarkBrown = 6,
}

impl From<u8> for HorseColor {
    fn from(value: u8) -> Self {
        match value {
            0 => HorseColor::White,
            1 => HorseColor::Creamy,
            2 => HorseColor::Chestnut,
            3 => HorseColor::Brown,
            4 => HorseColor::Black,
            5 => HorseColor::Gray,
            6 => HorseColor::DarkBrown,
            _ => HorseColor::White, // Default to white for invalid values
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum HorseStyle {
    #[default]
    None = 0,
    White = 1,
    Whitefield = 2,
    WhiteDots = 3,
    BlackDots = 4,
}

impl From<u8> for HorseStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => HorseStyle::None,
            1 => HorseStyle::White,
            2 => HorseStyle::Whitefield,
            3 => HorseStyle::WhiteDots,
            4 => HorseStyle::BlackDots,
            _ => HorseStyle::None,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum HorseArmor {
    #[default]
    None = 0,
    Iron = 1,
    Gold = 2,
    Diamond = 3,
}

impl From<u8> for HorseArmor {
    fn from(value: u8) -> Self {
        match value {
            0 => HorseArmor::None,
            1 => HorseArmor::Iron,
            2 => HorseArmor::Gold,
            3 => HorseArmor::Diamond,
            _ => HorseArmor::None,
        }
    }
}
