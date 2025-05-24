use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use super::{ByteArray, NbtTagType};

pub trait NBT {
    fn tag_type() -> NbtTagType {
        NbtTagType::Compound
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self>
    where
        Self: std::marker::Sized;
}

impl NBT for bool {
    fn tag_type() -> NbtTagType {
        NbtTagType::Byte
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(reader.read_i8()? == 1)
    }
}

impl NBT for i8 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Byte
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i8()
    }
}

impl NBT for i16 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Short
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i16::<BigEndian>()
    }
}

impl NBT for i32 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Int
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i32::<BigEndian>()
    }
}

impl NBT for i64 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Long
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i64::<BigEndian>()
    }
}

impl NBT for f32 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Float
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f32::<BigEndian>()
    }
}

impl NBT for f64 {
    fn tag_type() -> NbtTagType {
        NbtTagType::Double
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f64::<BigEndian>()
    }
}

impl NBT for ByteArray {
    fn tag_type() -> NbtTagType {
        NbtTagType::ByteArray
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let length = i32::deserialize_nbt(reader)?;
        let mut bytes = Vec::with_capacity(length as usize);
        for _ in 0..length {
            bytes.push(reader.read_i8()?);
        }
        Ok(ByteArray(bytes))
    }
}

impl NBT for String {
    fn tag_type() -> NbtTagType {
        NbtTagType::String
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let length = i16::deserialize_nbt(reader)?;

        let mut bytes = vec![0u8; length as usize];
        reader.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid UTF-8 sequence: {}", e),
            )
        })
    }
}

impl<T: NBT> NBT for Vec<T> {
    fn tag_type() -> NbtTagType {
        NbtTagType::List
    }

    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let _type = i8::deserialize_nbt(reader)?;
        let length = i32::deserialize_nbt(reader)?;

        let mut items = Vec::with_capacity(length as usize);

        for _ in 0..length {
            items.push(T::deserialize_nbt(reader)?);
        }

        Ok(items)
    }
}

impl NBT for NbtTagType {
    fn deserialize_nbt<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let raw = i8::deserialize_nbt(reader)?;
        NbtTagType::try_from(raw)
    }
}

pub fn deserialize_string_bytes<R: io::Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let length = i16::deserialize_nbt(reader)?;
    let mut bytes = vec![0u8; length as usize];
    reader.read_exact(&mut bytes)?;

    Ok(bytes)
}
