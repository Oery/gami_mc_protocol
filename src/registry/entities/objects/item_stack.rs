use gami_macros::entity;

use crate::packets::play::server::Item;

#[entity(Object)]
pub struct ItemStack {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub item: Item,
}

impl ItemStack {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Slot(10, value) = metadata {
                self.item = value.clone();
            }
        }
    }
}
