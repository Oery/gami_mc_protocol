use gami_macros::entity;

use crate::registry::entities::metadata::CollarColor;

#[entity(LivingEntity, Ageable, Tameable)]
pub struct Wolf {
    // Entity
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    // Tameable
    pub is_sitting: bool,
    pub is_tame: bool,
    pub owner_name: String,
    // Wolf
    pub is_angry: bool,
    pub health: f32,
    pub is_begging: bool,
    pub collar_color: CollarColor,
}

impl Wolf {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(16, value) => self.is_angry = (*value & 0x02) != 0,
                Metadata::Float(18, value) => self.health = *value,
                Metadata::Byte(19, value) => self.is_begging = *value != 0,
                Metadata::Byte(20, value) => self.collar_color = CollarColor::from(*value),
                _ => (),
            }
        }
    }
}
