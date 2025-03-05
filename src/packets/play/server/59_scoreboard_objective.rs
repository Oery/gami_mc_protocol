use gami_macros::packet;

#[packet(0x3B, server)]
pub struct ScoreboardObjective {
    pub objective: String,
    pub action: ScoreboardObjectiveAction,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum ScoreboardObjectiveAction {
    Add(ScoreboardData) = 0,
    Remove = 1,
    UpdateDisplayText(ScoreboardData) = 2,
}

impl Deserialize for ScoreboardObjectiveAction {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use ScoreboardObjectiveAction::*;

        match u8::deserialize(reader)? {
            0 => Ok(Add(ScoreboardData::deserialize(reader)?)),
            1 => Ok(Remove),
            2 => Ok(UpdateDisplayText(ScoreboardData::deserialize(reader)?)),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid scoreboard objective action".to_string(),
            )),
        }
    }
}

impl Serialize for ScoreboardObjectiveAction {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use ScoreboardObjectiveAction::*;

        match self {
            Add(data) => {
                u8::serialize(&0, buf)?;
                data.serialize(buf)?;
            }
            Remove => u8::serialize(&1, buf)?,
            UpdateDisplayText(data) => {
                u8::serialize(&2, buf)?;
                data.serialize(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreboardData {
    pub display_name: String,
    pub kind: ScoreboardKind,
}

#[derive(Default, Debug, PartialEq)]
pub enum ScoreboardKind {
    #[default]
    Integer,
    Hearts,
}

impl Deserialize for ScoreboardKind {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use ScoreboardKind::*;

        match String::deserialize(reader)?.as_str() {
            "integer" => Ok(Integer),
            "hearts" => Ok(Hearts),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid scoreboard kind".to_string(),
            )),
        }
    }
}

impl Serialize for ScoreboardKind {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use ScoreboardKind::*;

        match self {
            Integer => String::from("integer").serialize(buf),
            Hearts => String::from("hearts").serialize(buf),
        }
    }
}
