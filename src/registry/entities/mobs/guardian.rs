use gami_macros::entity;

#[entity(LivingEntity)]
pub struct Guardian {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub is_elder: bool,
    pub target_id: i32,
}

impl Guardian {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(16, value) => self.is_elder = *value != 0,
                Metadata::Int(17, value) => self.target_id = *value,
                _ => {}
            }
        }
    }
}
