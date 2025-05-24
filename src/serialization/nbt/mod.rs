mod deserializer;
mod serializer;
pub mod types;
mod validation;

pub use deserializer::*;
pub use serializer::*;
pub(crate) use validation::*;

pub struct ByteArray(pub Vec<i8>);

#[repr(i8)]
#[derive(Debug, PartialEq)]
pub enum NbtTagType {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl std::fmt::Display for NbtTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i8> for NbtTagType {
    type Error = std::io::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NbtTagType::End),
            1 => Ok(NbtTagType::Byte),
            2 => Ok(NbtTagType::Short),
            3 => Ok(NbtTagType::Int),
            4 => Ok(NbtTagType::Long),
            5 => Ok(NbtTagType::Float),
            6 => Ok(NbtTagType::Double),
            7 => Ok(NbtTagType::ByteArray),
            8 => Ok(NbtTagType::String),
            9 => Ok(NbtTagType::List),
            10 => Ok(NbtTagType::Compound),
            11 => Ok(NbtTagType::IntArray),
            12 => Ok(NbtTagType::LongArray),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid NBT Tag Type: {value}"),
            )),
        }
    }
}
