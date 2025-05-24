use gami_macros::entity;

#[entity(LivingEntity, Ageable)]
pub struct Mooshroom {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
}

impl Mooshroom {
    fn update(&mut self, _metadatas: &[Metadata]) {}
}
