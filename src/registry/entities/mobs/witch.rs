use gami_macros::entity;

#[entity(LivingEntity)]
pub struct Witch {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub is_aggressive: bool,
}

impl Witch {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(21, value) = metadata {
                self.is_aggressive = *value == 1
            }
        }
    }
}
