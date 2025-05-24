use derive_more::Display;
use gami_macros::{Deserialize, Serialize};

use crate::serialization::{Deserialize, Serialize};

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Display)]
#[encoding("u8")]
pub enum Enchantment {
    // Armor
    #[default]
    Protection = 0,
    FireProtection = 1,
    FeatherFalling = 2,
    BlastProtection = 3,
    ProjectileProtection = 4,
    Respiration = 5,
    AquaAffinity = 6,
    Thorns = 7,
    DepthStrider = 8,

    // Swords
    Sharpness = 16,
    Smite = 17,
    BaneOfArthropods = 18,
    Knockback = 19,
    FireAspect = 20,
    Looting = 21,

    // Tools
    Efficiency = 32,
    SilkTouch = 33,
    Unbreaking = 34,
    Fortune = 35,

    // Bows
    Power = 48,
    Punch = 49,
    Flame = 50,
    Infinity = 51,

    // Fishing Rod
    LuckOfTheSea = 61,
    Lure = 62,
}

impl From<i16> for Enchantment {
    fn from(value: i16) -> Self {
        match value {
            0 => Enchantment::Protection,
            1 => Enchantment::FireProtection,
            2 => Enchantment::FeatherFalling,
            3 => Enchantment::BlastProtection,
            4 => Enchantment::ProjectileProtection,
            5 => Enchantment::Respiration,
            6 => Enchantment::AquaAffinity,
            7 => Enchantment::Thorns,
            8 => Enchantment::DepthStrider,
            16 => Enchantment::Sharpness,
            17 => Enchantment::Smite,
            18 => Enchantment::BaneOfArthropods,
            19 => Enchantment::Knockback,
            20 => Enchantment::FireAspect,
            21 => Enchantment::Looting,
            32 => Enchantment::Efficiency,
            33 => Enchantment::SilkTouch,
            34 => Enchantment::Unbreaking,
            35 => Enchantment::Fortune,
            48 => Enchantment::Power,
            49 => Enchantment::Punch,
            50 => Enchantment::Flame,
            51 => Enchantment::Infinity,
            61 => Enchantment::LuckOfTheSea,
            62 => Enchantment::Lure,
            _ => panic!("Invalid enchantment"),
        }
    }
}
