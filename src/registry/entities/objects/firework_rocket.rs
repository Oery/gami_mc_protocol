use gami_macros::entity;

// TODO: This entity requires support for the Slot data type
#[entity(Object)]
pub struct FireworkRocket {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
}

impl FireworkRocket {
    fn update(&mut self, _metadatas: &[Metadata]) {}
}
