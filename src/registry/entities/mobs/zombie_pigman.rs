use gami_macros::entity;

#[entity(LivingEntity)]
pub struct ZombiePigman {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub is_baby: bool,
    pub is_villager: bool,
    pub is_converting: bool,
}

impl ZombiePigman {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(12, value) => self.is_baby = *value == 1,
                Metadata::Byte(13, value) => self.is_villager = *value == 1,
                Metadata::Byte(14, value) => self.is_converting = *value == 1,
                _ => (),
            }
        }
    }
}
