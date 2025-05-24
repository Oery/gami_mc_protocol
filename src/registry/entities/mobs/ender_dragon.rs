use gami_macros::entity;

#[entity(LivingEntity)]
pub struct EnderDragon {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
}

impl EnderDragon {
    fn update(&mut self, _metadatas: &[Metadata]) {}
}
