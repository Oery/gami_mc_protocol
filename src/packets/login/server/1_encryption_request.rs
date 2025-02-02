// use crate::packets::{Packet};
// use gami_macros::{packet, Deserialize, Serialize};

// use pkcs8::{DecodePublicKey, EncodePublicKey};
// use rsa::RsaPublicKey;
// use std::io::{Error, ErrorKind, Read, Result, Write};

// #[packet(0x01, server)]
// pub struct EncryptionRequest {
//     pub server_id: String,
//     pub public_key: RsaPublicKey,
//     pub verify_token: Vec<u8>,
// }

// impl Serialize for RsaPublicKey {
//     fn serialize(&self, buf: &mut dyn Write) -> Result<()> {
//         let bytes = RsaPublicKey::to_public_key_der(self)
//             .map_err(|e| Error::new(ErrorKind::Other, e))?
//             .as_bytes()
//             .to_vec();

//         bytes.serialize(buf)?;
//         Ok(())
//     }
// }

// impl Deserialize for RsaPublicKey {
//     fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
//         let bytes = Vec::deserialize(reader)?;
//         let key = RsaPublicKey::from_public_key_der(&bytes).unwrap();
//         Ok(key)
//     }
// }
