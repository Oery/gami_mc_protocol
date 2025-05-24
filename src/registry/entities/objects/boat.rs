use gami_macros::entity;

#[entity(Object)]
pub struct Boat {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    // Boat
    pub time_since_hit: i32,
    pub forward_direction: i32,
    pub damage_taken: f32,
}

impl Boat {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Int(17, value) => self.time_since_hit = *value,
                Metadata::Int(18, value) => self.forward_direction = *value,
                Metadata::Float(19, value) => self.damage_taken = *value,
                _ => (),
            }
        }
    }
}
