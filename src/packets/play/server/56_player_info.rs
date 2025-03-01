use gami_macros::packet;
use uuid::Uuid;

#[packet(0x38, server)]
pub struct PlayerInfo {
    pub action: PlayerInfoAction,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum PlayerInfoAction {
    AddPlayer(Vec<AddPlayer>),
    UpdateGameMode(Vec<UpdateGameMode>),
    UpdatePing(Vec<UpdatePing>),
    UpdateDisplayName(Vec<UpdateDisplayName>),
    RemovePlayer(Vec<RemovePlayer>),
}

impl Deserialize for PlayerInfoAction {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let action = deserialize_varint(reader)?;
        match action {
            0 => Ok(PlayerInfoAction::AddPlayer(Vec::deserialize(reader)?)),
            1 => Ok(PlayerInfoAction::UpdateGameMode(Vec::deserialize(reader)?)),
            2 => Ok(PlayerInfoAction::UpdatePing(Vec::deserialize(reader)?)),
            3 => Ok(PlayerInfoAction::UpdateDisplayName(Vec::deserialize(
                reader,
            )?)),
            4 => Ok(PlayerInfoAction::RemovePlayer(Vec::deserialize(reader)?)),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid player info action: {}", action),
            )),
        }
    }
}

impl Serialize for PlayerInfoAction {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        match self {
            PlayerInfoAction::AddPlayer(vec) => vec.serialize(buf),
            PlayerInfoAction::UpdateGameMode(vec) => vec.serialize(buf),
            PlayerInfoAction::UpdatePing(vec) => vec.serialize(buf),
            PlayerInfoAction::UpdateDisplayName(vec) => vec.serialize(buf),
            PlayerInfoAction::RemovePlayer(vec) => vec.serialize(buf),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddPlayer {
    pub uuid: Uuid,
    pub name: String,
    pub properties: Vec<Property>,
    #[encoding("varint")]
    pub game_mode: i32,
    #[encoding("varint")]
    pub ping: i32,
    pub has_display_name: bool,
    #[condition(self.has_display_name)]
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePing {
    pub uuid: Uuid,
    #[encoding("varint")]
    pub ping: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGameMode {
    pub uuid: Uuid,
    #[encoding("varint")]
    pub game_mode: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDisplayName {
    pub uuid: Uuid,
    pub has_display_name: bool,
    #[condition(self.has_display_name)]
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemovePlayer {
    pub uuid: Uuid,
}
