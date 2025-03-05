use gami_macros::packet;

use crate::registry::TextColor;

#[packet(0x3E, server)]
pub struct Teams {
    pub name: String,
    pub action: TeamsAction,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum TeamsAction {
    CreateTeam(CreateTeam) = 0,
    RemoveTeam = 1,
    UpdateTeam(UpdateTeam) = 2,
    AddPlayers(AddPlayers) = 3,
    RemovePlayers(RemovePlayers) = 4,
}

impl Deserialize for TeamsAction {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        match u8::deserialize(reader)? {
            0 => Ok(Self::CreateTeam(CreateTeam::deserialize(reader)?)),
            1 => Ok(Self::RemoveTeam),
            2 => Ok(Self::UpdateTeam(UpdateTeam::deserialize(reader)?)),
            3 => Ok(Self::AddPlayers(AddPlayers::deserialize(reader)?)),
            4 => Ok(Self::RemovePlayers(RemovePlayers::deserialize(reader)?)),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid teams action".to_string(),
            )),
        }
    }
}

impl Serialize for TeamsAction {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use TeamsAction::*;

        match self {
            CreateTeam(data) => {
                u8::serialize(&0, buf)?;
                data.serialize(buf)?;
            }
            RemoveTeam => u8::serialize(&1, buf)?,
            UpdateTeam(data) => {
                u8::serialize(&2, buf)?;
                data.serialize(buf)?;
            }
            AddPlayers(data) => {
                u8::serialize(&3, buf)?;
                data.serialize(buf)?;
            }
            RemovePlayers(data) => {
                u8::serialize(&4, buf)?;
                data.serialize(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTeam {
    pub display_name: String,
    pub prefix: String,
    pub suffix: String,
    pub friendly_fire: FriendlyFire,
    pub nametag_visibility: String,
    pub color: TextColor,
    pub players: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTeam {
    pub display_name: String,
    pub prefix: String,
    pub suffix: String,
    pub friendly_fire: FriendlyFire,
    pub nametag_visibility: String,
    pub color: TextColor,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddPlayers {
    pub players: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemovePlayers {
    pub players: Vec<String>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[encoding("u8")]
pub enum FriendlyFire {
    Off = 0,
    On = 1,
    #[default]
    SeeInvisibleFriends = 3,
}

#[derive(Debug, PartialEq)]
pub enum NametagVisibility {
    Always,
    HideForOtherTeams,
    HideForOwnTeam,
    Never,
}

impl Deserialize for NametagVisibility {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use NametagVisibility::*;

        match String::deserialize(reader)?.as_str() {
            "always" => Ok(Always),
            "hideForOtherTeams" => Ok(HideForOtherTeams),
            "hideForOwnTeam" => Ok(HideForOwnTeam),
            "never" => Ok(Never),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid nametag visibility".to_string(),
            )),
        }
    }
}

impl Serialize for NametagVisibility {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use NametagVisibility::*;

        match self {
            Always => String::from("always").serialize(buf),
            HideForOtherTeams => String::from("hideForOtherTeams").serialize(buf),
            HideForOwnTeam => String::from("hideForOwnTeam").serialize(buf),
            Never => String::from("never").serialize(buf),
        }
    }
}
