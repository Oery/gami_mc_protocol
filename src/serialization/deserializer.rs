use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;

use super::deserialize_varint;

pub trait Deserialize {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self>
    where
        Self: std::marker::Sized;
}

// PRIMITIVES

impl Deserialize for bool {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        Ok(byte == 1)
    }
}

impl Deserialize for u8 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u8()
    }
}

impl Deserialize for u16 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u16::<BigEndian>()
    }
}

impl Deserialize for u32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u32::<BigEndian>()
    }
}

impl Deserialize for u64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u64::<BigEndian>()
    }
}

impl Deserialize for i8 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i8()
    }
}

impl Deserialize for i16 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i16::<BigEndian>()
    }
}

impl Deserialize for i32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i32::<BigEndian>()
    }
}

impl Deserialize for i64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i64::<BigEndian>()
    }
}

impl Deserialize for f32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f32::<BigEndian>()
    }
}

impl Deserialize for f64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f64::<BigEndian>()
    }
}

impl Deserialize for String {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let length = deserialize_varint(reader)?;

        let mut str_bytes = vec![0u8; length as usize];
        reader.read_exact(&mut str_bytes)?;

        String::from_utf8(str_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let value = T::deserialize(reader)?;
        Ok(Some(value))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let length = deserialize_varint(reader)?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(T::deserialize(reader)?);
        }
        Ok(vec)
    }
}

impl Deserialize for Uuid {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let most_significant = reader.read_u64::<BigEndian>()?;
        let least_significant = reader.read_u64::<BigEndian>()?;
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes[..8].copy_from_slice(&most_significant.to_be_bytes());
        uuid_bytes[8..].copy_from_slice(&least_significant.to_be_bytes());
        let uuid = Uuid::from_bytes(uuid_bytes);
        Ok(uuid)
    }
}
