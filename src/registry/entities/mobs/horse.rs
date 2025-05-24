use gami_macros::entity;

use crate::registry::entities::metadata::HorseArmor;
use crate::registry::entities::metadata::HorseColor;
use crate::registry::entities::metadata::HorseKind;
use crate::registry::entities::metadata::HorseStyle;

#[entity(LivingEntity, Ageable)]
pub struct Horse {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub kind: HorseKind,
    pub color: HorseColor,
    pub style: HorseStyle,
    pub armor: HorseArmor,
    pub owner_name: String,

    pub is_tamed: bool,
    pub has_saddle: bool,
    pub has_chest: bool,
    pub is_bred: bool,
    pub is_eating: bool,
    pub is_rearing: bool,
    pub mouth_open: bool,
}

impl Horse {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Int(16, value) => {
                    self.kind = HorseKind::from((value & 0xFF) as u8);
                    self.is_tamed = (value & 0x02) != 0;
                    self.has_saddle = (value & 0x04) != 0;
                    self.has_chest = (value & 0x08) != 0;
                    self.is_bred = (value & 0x10) != 0;
                    self.is_eating = (value & 0x20) != 0;
                    self.is_rearing = (value & 0x40) != 0;
                    self.mouth_open = (value & 0x80) != 0;
                }
                Metadata::Int(19, value) => {
                    self.color = HorseColor::from((value & 0xFF) as u8);
                    self.style = HorseStyle::from(((value >> 8) & 0xFF) as u8);
                }
                Metadata::Int(20, value) => {
                    self.armor = HorseArmor::from((value & 0xFF) as u8);
                }
                Metadata::String(21, value) => self.owner_name = value.clone(),
                _ => (),
            }
        }
    }
}
