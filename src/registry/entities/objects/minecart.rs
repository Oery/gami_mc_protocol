use gami_macros::entity;

#[entity(Object)]
pub struct Minecart {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    // Minecart
    pub shaking_power: i32,
    pub shaking_direction: i32,
    pub damage_taken: f32,
    pub block_id: u8,
    pub block_data: u8,
    pub block_y_position: i32,
    pub show_block: bool,
}

impl Minecart {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Int(17, value) => self.shaking_power = *value,
                Metadata::Int(18, value) => self.shaking_direction = *value,
                Metadata::Float(19, value) => self.damage_taken = *value,
                Metadata::Int(20, value) => {
                    self.block_id = (*value & 0x00FF) as u8;
                    self.block_data = ((*value & 0xFF00) >> 8) as u8;
                }
                Metadata::Int(21, value) => self.block_y_position = *value,
                Metadata::Byte(22, value) => self.show_block = *value != 0,
                _ => (),
            }
        }
    }
}
