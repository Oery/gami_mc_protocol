use std::io::{self, Write};

use byteorder::{BigEndian, WriteBytesExt};
use uuid::Uuid;

use super::encoding::varint::serialize_varint_i32;

pub trait Serialize {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()>;
}

// PRIMITIVES

impl Serialize for bool {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&[if *self { 1 } else { 0 }])?;
        Ok(())
    }
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for u32 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for u64 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i8 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i16 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for f32 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for f64 {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        let bytes = &self.as_bytes();
        serialize_varint_i32(&(bytes.len() as i32), buf)?;
        buf.write_all(bytes)?;

        Ok(())
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        if let Some(value) = self {
            value.serialize(buf)?;
        }
        Ok(())
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        serialize_varint_i32(&(self.len() as i32), buf)?;
        for item in self {
            item.serialize(buf)?;
        }
        Ok(())
    }
}

impl Serialize for Uuid {
    fn serialize(&self, buf: &mut dyn Write) -> io::Result<()> {
        let uuid_bytes = &self.as_bytes();
        let most_significant = u64::from_be_bytes(uuid_bytes[..8].try_into().unwrap());
        let least_significant = u64::from_be_bytes(uuid_bytes[8..].try_into().unwrap());

        buf.write_u64::<BigEndian>(most_significant)?;
        buf.write_u64::<BigEndian>(least_significant)?;

        Ok(())
    }
}
