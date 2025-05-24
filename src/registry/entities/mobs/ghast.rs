use gami_macros::entity;

#[entity(LivingEntity)]
pub struct Ghast {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub is_attacking: bool,
}

impl Ghast {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(16, value) = metadata {
                self.is_attacking = *value != 0
            }
        }
    }
}
