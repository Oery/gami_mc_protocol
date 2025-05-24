use std::io;

use gami_macros::packet;

use crate::serialization::nbt::types::item::ItemProperties;
use crate::serialization::nbt::NBT;

#[packet(0x2F, server)]
pub struct SetSlot {
    pub window_id: i8,
    pub slot: i16,
    #[encoding("custom")]
    pub item: Option<Item>,
}

#[derive(Serialize, PartialEq, Debug, Default, Clone)]
pub struct Item {
    pub id: i16,
    #[encoding("varint")]
    pub count: i32,
    pub damage: i16,
    pub properties: Option<ItemProperties>,
}

impl Deserialize for Option<Item> {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut item = Item {
            id: i16::deserialize_nbt(reader)?,
            ..Default::default()
        };

        if item.id == -1 {
            return Ok(None);
        }

        item.count = deserialize_varint_i32(reader)?;
        item.damage = i16::deserialize(reader)?;
        item.properties = Option::<ItemProperties>::deserialize(reader)?;

        Ok(Some(item))
    }
}

impl Deserialize for Item {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            id: i16::deserialize(reader)?,
            count: deserialize_varint_i32(reader)?,
            damage: i16::deserialize(reader)?,
            properties: Option::<ItemProperties>::deserialize(reader)?,
        })
    }
}
