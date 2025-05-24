use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// TODO: Add support for newer versions of Minecraft because it changed at some point

pub fn serialize_fixed_point_f64(x: &f64, writer: &mut dyn Write) -> io::Result<()> {
    let x_fixed = (x * (1 << 5) as f64) as i32;
    writer.write_i32::<BigEndian>(x_fixed)?;
    Ok(())
}

pub fn deserialize_fixed_point_f64<R: Read>(reader: &mut R) -> io::Result<f64> {
    let x_fixed = reader.read_i32::<BigEndian>()?;
    Ok(x_fixed as f64 / (1 << 5) as f64)
}

pub fn serialize_fixed_point_f32(x: &f32, writer: &mut dyn Write) -> io::Result<()> {
    let scaled = (x * 32.0).round() as i32;
    let clamped = scaled.clamp(i8::MIN as i32, i8::MAX as i32);
    writer.write_i8(clamped as i8)?;
    Ok(())
}

pub fn deserialize_fixed_point_f32<R: Read>(reader: &mut R) -> io::Result<f32> {
    let x_fixed = reader.read_i8()? as f32 / 32.0;
    Ok(x_fixed)
}

pub fn serialize_fixed_point_i16(x: &i16, writer: &mut dyn Write) -> io::Result<()> {
    let x_fixed = x * (1 << 5);
    writer.write_i16::<BigEndian>(x_fixed)?;
    Ok(())
}

pub fn deserialize_fixed_point_i16<R: Read>(reader: &mut R) -> io::Result<i16> {
    let x_fixed = reader.read_i16::<BigEndian>()?;
    Ok(x_fixed / (1 << 5))
}

pub fn serialize_fixed_point_i32(x: &i32, writer: &mut dyn Write) -> io::Result<()> {
    let x_fixed = x * (1 << 5);
    writer.write_i32::<BigEndian>(x_fixed)?;
    Ok(())
}

pub fn deserialize_fixed_point_i32<R: Read>(reader: &mut R) -> io::Result<i32> {
    let x_fixed = reader.read_i32::<BigEndian>()?;
    Ok(x_fixed / (1 << 5))
}
