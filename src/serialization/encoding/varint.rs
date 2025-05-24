use std::io::{self, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

// TODO: Check why we use this instead of cursor.write_varint() which is implemented in this file
pub fn serialize_varint_i32(value: &i32, writer: &mut dyn Write) -> io::Result<()> {
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

pub fn deserialize_varint_i32<R: io::Read>(reader: &mut R) -> io::Result<i32> {
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
    fn read_varint_len(&mut self) -> io::Result<(usize, usize)>;
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

    fn read_varint_len(&mut self) -> io::Result<(usize, usize)> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut length: usize = 0;

        while let Ok(byte) = self.read_u8() {
            length += 1;

            value |= ((byte & 0b0111_1111) as i32) << position;

            if byte & 0b1000_0000 == 0 {
                return Ok((value as usize, length));
            }

            position += 7;
            if position >= 32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "VarInt is too big",
        ))
    }
}

pub fn decode_varint_length(buf: &[u8]) -> io::Result<(usize, usize)> {
    let mut value: i32 = 0;
    let mut shift = 0;

    for (i, &byte) in buf.iter().enumerate() {
        let seven_bits = (byte & 0x7F) as i32;
        value |= seven_bits << shift;

        // MSB==0 means “this was the last byte”
        if byte & 0x80 == 0 {
            return Ok((value as usize, i + 1));
        }

        shift += 7;
        if shift >= 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt is too big (over 32 bits)",
            ));
        }
    }

    // Buffer ended before we saw a byte with MSB=0
    Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "Buffer too small for VarInt",
    ))
}

pub trait ToVarInt {
    fn to_varint(&self) -> io::Result<Vec<u8>>;
}

impl ToVarInt for i32 {
    fn to_varint(&self) -> io::Result<Vec<u8>> {
        let mut writer = std::io::Cursor::new(Vec::new());
        serialize_varint_i32(self, &mut writer)?;
        Ok(writer.into_inner())
    }
}

pub fn deserialize_varint_vec<R: io::Read>(reader: &mut R) -> io::Result<Vec<i32>> {
    let length = deserialize_varint_i32(reader)?;
    let mut vec = Vec::with_capacity(length as usize);
    for _ in 0..length {
        vec.push(deserialize_varint_i32(reader)?);
    }
    Ok(vec)
}

pub fn serialize_varint_vec(vec: &Vec<i32>, writer: &mut dyn io::Write) -> io::Result<()> {
    serialize_varint_i32(&(vec.len() as i32), writer)?;
    for item in vec {
        serialize_varint_i32(item, writer)?;
    }
    Ok(())
}

pub fn serialize_varint_option(option: &Option<i32>, writer: &mut dyn io::Write) -> io::Result<()> {
    if let Some(value) = option {
        serialize_varint_i32(value, writer)?;
    }
    Ok(())
}
