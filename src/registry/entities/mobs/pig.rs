use gami_macros::entity;

#[entity(LivingEntity, Ageable)]
pub struct Pig {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub has_saddle: bool,
}

impl Pig {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(16, value) = metadata {
                self.has_saddle = *value != 0
            }
        }
    }
}
