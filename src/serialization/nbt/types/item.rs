use std::io;

use crate::registry::Enchantment as EnchantmentKind;
use crate::serialization::nbt::deserialize_string_bytes;
use crate::serialization::nbt::{NbtTagType, NBT};
use crate::serialization::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ItemProperties {
    pub ench: Vec<Enchantment>,
    pub display: Option<DisplayProperties>,
    pub unbreakable: bool,
}

impl Deserialize for ItemProperties {
    fn deserialize<R: io::Read>(_reader: &mut R) -> io::Result<Self> {
        panic!("This should not be called");
    }
}

impl Deserialize for Option<ItemProperties> {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match NbtTagType::deserialize_nbt(reader)? {
            NbtTagType::Compound => {
                // Skip Name (root tag has no name)
                let _ = i16::deserialize_nbt(reader)?;
                Ok(Some(ItemProperties::deserialize_nbt(reader)?))
            }

            NbtTagType::End => Ok(None),

            tag => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected End or Compound but found {tag}"),
            )),
        }
    }
}

impl Serialize for Option<ItemProperties> {
    fn serialize(&self, buf: &mut dyn io::Write) -> io::Result<()> {
        unimplemented!()
    }
}

impl NBT for ItemProperties {
    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut item = Self { ..Self::default() };

        loop {
            if NbtTagType::deserialize_nbt(reader)? == NbtTagType::End {
                break;
            }

            match deserialize_string_bytes(reader)?.as_slice() {
                b"ench" => item.ench = Vec::<Enchantment>::deserialize_nbt(reader)?,
                b"display" => item.display = Some(DisplayProperties::deserialize_nbt(reader)?),
                b"Unbreakable" => item.unbreakable = bool::deserialize_nbt(reader)?,

                field => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Unknown NBT Tag in Item tag: {}",
                            String::from_utf8_lossy(field)
                        ),
                    ))
                }
            }
        }

        Ok(item)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DisplayProperties {
    pub name: Option<String>,
    pub lore: Option<Vec<String>>,
}

impl NBT for DisplayProperties {
    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut name = None;
        let mut lore = None;

        loop {
            if NbtTagType::deserialize_nbt(reader)? == NbtTagType::End {
                break;
            }

            match deserialize_string_bytes(reader)?.as_slice() {
                b"Name" => name = Some(String::deserialize_nbt(reader)?),
                b"Lore" => lore = Some(Vec::<String>::deserialize_nbt(reader)?),

                field => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Unknown NBT Tag in Display tag: {}",
                            String::from_utf8_lossy(field)
                        ),
                    ))
                }
            };
        }

        Ok(Self { lore, name })
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Enchantment {
    pub id: EnchantmentKind,
    pub lvl: i16,
}

impl NBT for EnchantmentKind {
    fn tag_type() -> NbtTagType {
        NbtTagType::Short
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let id = i16::deserialize_nbt(reader)?;
        Ok(EnchantmentKind::from(id))
    }
}

// impl NBT for Enchantment {
//     fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
//         validate_field(reader, b"lvl", NbtTagType::Short)?;
//         let lvl = i16::deserialize_nbt(reader)?;

//         validate_field(reader, b"id", NbtTagType::Short)?;
//         let id = EnchantmentKind::deserialize_nbt(reader)?;

//         if NbtTagType::deserialize_nbt(reader)? != NbtTagType::End {
//             return Err(io::Error::new(
//                 io::ErrorKind::InvalidData,
//                 "Expected End Marker but found something else",
//             ));
//         }

//         Ok(Enchantment { id, lvl })
//     }
// }

impl NBT for Enchantment {
    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut lvl = None;
        let mut id = None;

        loop {
            if NbtTagType::deserialize_nbt(reader)? == NbtTagType::End {
                break;
            }

            match deserialize_string_bytes(reader)?.as_slice() {
                b"lvl" => lvl = Some(i16::deserialize_nbt(reader)?),
                b"id" => id = Some(EnchantmentKind::deserialize_nbt(reader)?),

                field => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Unknown NBT Tag in Enchantment tag: {}",
                            String::from_utf8_lossy(field)
                        ),
                    ))
                }
            };
        }

        if let Some((lvl, id)) = lvl.zip(id) {
            return Ok(Enchantment { id, lvl });
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid Enchantment",
        ))
    }
}
