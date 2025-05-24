use gami_macros::entity;

#[entity(LivingEntity, Ageable)]
pub struct Villager {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub age: u8,
    pub job: Job,
}

impl Villager {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Int(16, value) = metadata {
                self.job = Job::from(*value as u8)
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Job {
    #[default]
    Farmer = 0,
    Librarian = 1,
    Priest = 2,
    Blacksmith = 3,
    Butcher = 4,
}

impl From<u8> for Job {
    fn from(value: u8) -> Self {
        match value {
            0 => Job::Farmer,
            1 => Job::Librarian,
            2 => Job::Priest,
            3 => Job::Blacksmith,
            4 => Job::Butcher,
            _ => Job::Farmer, // Default to farmer for invalid values
        }
    }
}
