#[derive(Debug, Default, PartialEq)]
pub enum CreeperState {
    #[default]
    Idle = -1,
    Fuse = 1,
}

impl From<i8> for CreeperState {
    fn from(value: i8) -> Self {
        match value {
            -1 => CreeperState::Idle,
            1 => CreeperState::Fuse,
            _ => panic!("Invalid Creeper State"),
        }
    }
}
