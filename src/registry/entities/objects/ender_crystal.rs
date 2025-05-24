use gami_macros::entity;

#[entity(Object)]
pub struct EnderCrystal {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub health: i32,
}

impl EnderCrystal {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Int(8, value) = metadata {
                self.health = *value
            }
        }
    }
}
