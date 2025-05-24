use gami_macros::entity;

#[entity(LivingEntity)]
pub struct WitherBoss {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub watched_target_1: i32,
    pub watched_target_2: i32,
    pub watched_target_3: i32,
    pub invulnerable_time: i32,
}

impl WitherBoss {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Int(17, value) => self.watched_target_1 = *value,
                Metadata::Int(18, value) => self.watched_target_2 = *value,
                Metadata::Int(19, value) => self.watched_target_3 = *value,
                Metadata::Int(20, value) => self.invulnerable_time = *value,
                _ => (),
            }
        }
    }
}
