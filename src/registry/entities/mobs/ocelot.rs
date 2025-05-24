use gami_macros::entity;

#[entity(LivingEntity, Ageable, Tameable)]
pub struct Ocelot {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub is_sitting: bool,
    pub is_tame: bool,
    pub owner_name: String,
    pub ocelot_type: u8,
}

impl Ocelot {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(18, value) = metadata {
                self.ocelot_type = *value
            }
        }
    }
}
