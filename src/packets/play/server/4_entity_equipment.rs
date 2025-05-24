use gami_macros::packet;

use crate::packets::play::server::Item;

#[packet(0x04, server)]
pub struct EntityEquipment {
    #[encoding("varint")]
    pub entity_id: i32,
    /// 1: boots, 2: leggings, 3: chestplate, 4: helmet
    pub slot: i16,
    pub item: Item,
}
