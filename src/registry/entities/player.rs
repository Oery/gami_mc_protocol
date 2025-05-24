use gami_macros::entity;

#[entity(LivingEntity)]
pub struct Player {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,

    pub skin_flags: u8,
    pub absorbtion_health: f32,
    pub score: i32,
}

impl Player {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(10, value) => self.skin_flags = *value,
                // There is a value for byte 16 but it is unused
                Metadata::Float(17, value) => self.absorbtion_health = *value,
                Metadata::Int(18, value) => self.score = *value,
                _ => (),
            }
        }
    }
}
