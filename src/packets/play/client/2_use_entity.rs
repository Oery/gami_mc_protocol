use gami_macros::packet;

#[packet(0x02, client)]
pub struct UseEntity {
    #[encoding("varint")]
    pub target: i32,
    pub interaction_type: InteractionType,
}

#[derive(Debug, PartialEq)]
pub enum InteractionType {
    Interact,
    Attack,
    InteractAt { x: f32, y: f32, z: f32 },
}

impl UseEntity {
    pub fn attack(entity_id: i32) -> Self {
        Self {
            target: entity_id,
            interaction_type: InteractionType::Attack,
        }
    }
}

impl Deserialize for InteractionType {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use InteractionType::*;

        let action = deserialize_varint_i32(reader)?;
        match action {
            0 => Ok(Interact),
            1 => Ok(Attack),
            2 => Ok(InteractAt {
                x: f32::deserialize(reader)?,
                y: f32::deserialize(reader)?,
                z: f32::deserialize(reader)?,
            }),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid use entity type : {action}"),
            )),
        }
    }
}

impl Serialize for InteractionType {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use InteractionType::*;

        match self {
            Interact => serialize_varint_i32(&0, buf),
            Attack => serialize_varint_i32(&1, buf),
            InteractAt { x, y, z } => {
                serialize_varint_i32(&2, buf)?;
                x.serialize(buf)?;
                y.serialize(buf)?;
                z.serialize(buf)?;
                Ok(())
            }
        }
    }
}
