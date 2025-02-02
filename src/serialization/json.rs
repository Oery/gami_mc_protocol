use std::io;

use super::{Deserialize, Serialize};

pub fn deserialize_json<R, T>(reader: &mut R) -> io::Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    let bytes = String::deserialize(reader)?;
    let data: T = serde_json::from_str(&bytes)?;
    Ok(data)
}

pub fn serialize_json<T: serde::Serialize>(
    value: &T,
    writer: &mut dyn io::Write,
) -> io::Result<()> {
    let bytes = serde_json::to_string(value)?;
    bytes.serialize(writer)
}
