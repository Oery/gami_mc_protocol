use std::io::{Error, ErrorKind, Read};

use crate::serialization::nbt::{NbtTagType, NBT};
use crate::serialization::Deserialize;

pub fn validate_field<R: Read>(reader: &mut R, name: &[u8], tag: NbtTagType) -> Result<(), Error> {
    // Validate NBT Tag Type
    let result = NbtTagType::deserialize_nbt(reader)?;
    if result != tag {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Expected {tag}, got {result}"),
        ));
    }

    // Validate Field Name
    let length = i16::deserialize(reader)?;

    let mut field = vec![0u8; length as usize];
    reader.read_exact(&mut field)?;

    if field.as_slice() != name {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Expected field {}, got {}",
                String::from_utf8_lossy(name),
                String::from_utf8_lossy(&field)
            ),
        ));
    }

    Ok(())
}
