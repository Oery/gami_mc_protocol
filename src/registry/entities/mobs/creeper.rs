use gami_macros::entity;

use crate::registry::entities::metadata::CreeperState;

#[entity(LivingEntity)]
pub struct Creeper {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub state: CreeperState,
    pub is_powered: bool,
}

impl Creeper {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(16, value) => self.state = CreeperState::from(*value as i8),
                Metadata::Byte(17, value) => self.is_powered = *value == 1,
                _ => (),
            }
        }
    }
}
