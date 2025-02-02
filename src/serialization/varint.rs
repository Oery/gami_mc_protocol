use std::io::{self, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::{Deserialize, Serialize};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

// TODO: Check why we use this instead of cursor.write_varint() which is implemented in this file
pub fn serialize_varint(value: &i32, writer: &mut dyn Write) -> io::Result<()> {
    let mut value = *value as u32;
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        writer.write_u8(byte)?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub fn deserialize_varint<R: io::Read>(reader: &mut R) -> io::Result<i32> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let byte = reader.read_u8()?;
        result |= ((byte & 0x7F) as i32) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

pub trait VarIntReader {
    fn read_varint(&mut self) -> io::Result<i32>;
    fn read_varint_full(&mut self) -> io::Result<(i32, Vec<u8>)>;
}

impl<R: std::io::Read> VarIntReader for R {
    fn read_varint(&mut self) -> io::Result<i32> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut buffer = [0u8; 1];

        loop {
            self.read_exact(&mut buffer)?;
            let byte = buffer[0];

            value |= ((byte & 0b0111_1111) as i32) << position;
            if byte & 0b1000_0000 == 0 {
                return io::Result::Ok(value);
            }
            position += 7;
            if position >= 32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }
        }
    }

    fn read_varint_full(&mut self) -> io::Result<(i32, Vec<u8>)> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut buffer = [0u8; 1];
        let mut bytes = Vec::new();

        loop {
            self.read_exact(&mut buffer)?;
            let byte = buffer[0];

            value |= ((byte & 0b0111_1111) as i32) << position;
            bytes.push(byte);
            if byte & 0b1000_0000 == 0 {
                return io::Result::Ok((value, bytes));
            }
            position += 7;
            if position >= 32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }
        }
    }
}

pub trait VarIntWriter {
    fn write_varint(&mut self, value: i32) -> io::Result<()>;
}

impl<W: std::io::Write> VarIntWriter for W {
    fn write_varint(&mut self, mut value: i32) -> io::Result<()> {
        loop {
            let mut temp = (value & SEGMENT_BITS as i32) as u8;
            value >>= 7;

            if value != 0 {
                temp |= CONTINUE_BIT;
            }

            self.write_all(&[temp])?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

pub trait ToVarInt {
    fn to_varint(&self) -> io::Result<Vec<u8>>;
}

impl ToVarInt for i32 {
    fn to_varint(&self) -> io::Result<Vec<u8>> {
        let mut writer = std::io::Cursor::new(Vec::new());
        serialize_varint(self, &mut writer)?;
        Ok(writer.into_inner())
    }
}

pub fn deserialize_varint_vec<R: io::Read>(reader: &mut R) -> io::Result<Vec<i32>> {
    let length = deserialize_varint(reader)?;
    let mut vec = Vec::with_capacity(length as usize);
    for _ in 0..length {
        vec.push(deserialize_varint(reader)?);
    }
    Ok(vec)
}

pub fn serialize_varint_vec(vec: &Vec<i32>, writer: &mut dyn io::Write) -> io::Result<()> {
    serialize_varint(&(vec.len() as i32), writer)?;
    for item in vec {
        serialize_varint(item, writer)?;
    }
    Ok(())
}

pub fn serialize_varint_option(option: &Option<i32>, writer: &mut dyn io::Write) -> io::Result<()> {
    if let Some(value) = option {
        serialize_varint(value, writer)?;
    }
    Ok(())
}

pub struct VarInt<T>(T);

impl Serialize for VarInt<i32> {
    fn serialize(&self, buf: &mut dyn io::Write) -> io::Result<()> {
        serialize_varint(&self.0, buf)
    }
}

impl Deserialize for VarInt<i32> {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(VarInt(deserialize_varint(reader)?))
    }
}
