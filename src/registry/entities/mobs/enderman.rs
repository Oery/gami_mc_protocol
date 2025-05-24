use gami_macros::entity;

#[entity(LivingEntity)]
pub struct Enderman {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub carried_block: i16,
    pub carried_block_data: u8,
    pub is_screaming: bool,
}

impl Enderman {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Short(16, value) => self.carried_block = *value,
                Metadata::Byte(17, value) => self.carried_block_data = *value,
                Metadata::Byte(18, value) => self.is_screaming = *value == 1,
                _ => (),
            }
        }
    }
}
