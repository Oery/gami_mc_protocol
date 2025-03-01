use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Metadata {
    Byte(u8, u8),
    Short(u8, i16),
    Int(u8, i32),
    Float(u8, f32),
    String(u8, String),
    Slot(u8),
    Xyz(u8, i32, i32, i32),
    Pyr(u8, f32, f32, f32),
    End,
}

impl Deserialize for Metadata {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let Ok(item) = reader.read_u8() else {
            return Ok(Metadata::End);
        };

        if item == 0x7F {
            return Ok(Metadata::End);
        }

        let index = item & 0x1F;
        let metadata = match item >> 5 {
            0 => {
                let byte = reader.read_u8()?;
                Metadata::Byte(index, byte)
            }

            1 => {
                let short = reader.read_i16::<BigEndian>()?;
                Metadata::Short(index, short)
            }

            2 => {
                let int = reader.read_i32::<BigEndian>()?;
                Metadata::Int(index, int)
            }

            3 => {
                let float = reader.read_f32::<BigEndian>()?;
                Metadata::Float(index, float)
            }

            4 => {
                let string = String::deserialize(reader)?;
                Metadata::String(index, string)
            }

            5 => {
                // TODO: Implement slot
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Slot metadata not implemented",
                ));
            }

            6 => {
                let x = reader.read_i32::<BigEndian>()?;
                let y = reader.read_i32::<BigEndian>()?;
                let z = reader.read_i32::<BigEndian>()?;
                Metadata::Xyz(index, x, y, z)
            }

            7 => {
                let pitch = reader.read_f32::<BigEndian>()?;
                let yaw = reader.read_f32::<BigEndian>()?;
                let roll = reader.read_f32::<BigEndian>()?;
                Metadata::Pyr(index, pitch, yaw, roll)
            }

            _ => panic!("Invalid metadata kind"),
        };

        Ok(metadata)
    }
}

impl Serialize for Metadata {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        // TODO: Implement serialize
        todo!("Serialize metadata");

        Ok(())
    }
}

pub fn deserialize_metadatas_vec<R: Read>(reader: &mut R) -> io::Result<Vec<Metadata>> {
    let mut vec = Vec::with_capacity(10);
    loop {
        let metadata = Metadata::deserialize(reader)?;
        if metadata == Metadata::End {
            break;
        }
        vec.push(metadata);
    }
    Ok(vec)
}

pub fn serialize_metadatas_vec(metadatas: &[Metadata], writer: &mut dyn Write) -> io::Result<()> {
    for metadata in metadatas {
        metadata.serialize(writer)?;
    }
    Ok(())
}
