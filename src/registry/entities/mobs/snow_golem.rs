use gami_macros::entity;

#[entity(LivingEntity)]
pub struct SnowGolem {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
}

impl SnowGolem {
    fn update(&mut self, _metadatas: &[Metadata]) {}
}
