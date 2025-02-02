use crate::packets::Packet;
use crate::serialization::{deserialize_json, serialize_json};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x00, server)]
pub struct ServerInfo {
    #[encoding("json")]
    pub server_info: ServerInfoPayload,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct ServerInfoPayload {
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<ServerStatusPlayers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct ServerStatusPlayers {
    pub max: u32,
    pub online: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<Player>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub id: String,
}
