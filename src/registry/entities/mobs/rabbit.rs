use gami_macros::entity;

#[entity(LivingEntity, Ageable)]
pub struct Rabbit {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub kind: RabbitKind,
}

impl Rabbit {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(18, value) = metadata {
                self.kind = RabbitKind::from(*value as i8)
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum RabbitKind {
    #[default]
    Brown = 0,
    White = 1,
    Black = 2,
    BlackAndWhite = 3,
    Gold = 4,
    SaltAndPepper = 5,
    KillerBunny = 99,
}

impl From<i8> for RabbitKind {
    fn from(value: i8) -> Self {
        match value {
            0 => RabbitKind::Brown,
            1 => RabbitKind::White,
            2 => RabbitKind::Black,
            3 => RabbitKind::BlackAndWhite,
            4 => RabbitKind::Gold,
            5 => RabbitKind::SaltAndPepper,
            99 => RabbitKind::KillerBunny,
            _ => RabbitKind::Brown,
        }
    }
}
