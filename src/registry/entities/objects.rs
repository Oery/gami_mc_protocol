use crate::serialization::{Deserialize, Serialize};
use gami_macros::{Deserialize, Serialize};

mod activated_tnt;
mod area_effect_cloud;
mod armor_stand;
mod arrow;
mod boat;
mod egg;
mod ender_crystal;
mod ender_pearl;
mod exp_bottle;
mod experience_orb;
mod eye_of_ender;
mod falling_dragon_egg;
mod falling_objects;
mod fire_charge;
mod fireball;
mod firework_rocket;
mod fishing_float;
mod item_frame;
mod item_stack;
mod leash_knot;
mod minecart;
mod potion;
mod snowball;
mod wither_skull;

pub use activated_tnt::ActivatedTnt;
pub use area_effect_cloud::AreaEffectCloud;
pub use armor_stand::ArmorStand;
pub use arrow::Arrow;
pub use boat::Boat;
pub use egg::Egg;
pub use ender_crystal::EnderCrystal;
pub use ender_pearl::EnderPearl;
pub use exp_bottle::ExpBottle;
pub use experience_orb::ExperienceOrb;
pub use eye_of_ender::EyeOfEnder;
pub use falling_dragon_egg::FallingDragonEgg;
pub use falling_objects::FallingObjects;
pub use fire_charge::FireCharge;
pub use fireball::Fireball;
pub use firework_rocket::FireworkRocket;
pub use fishing_float::FishingFloat;
pub use item_frame::ItemFrame;
pub use item_stack::ItemStack;
pub use leash_knot::LeashKnot;
pub use minecart::Minecart;
pub use potion::Potion;
pub use snowball::Snowball;
pub use wither_skull::WitherSkull;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[encoding("u8")]
pub enum ObjectKind {
    Boat = 1,
    ItemStack = 2,
    AreaEffectCloud = 3,
    Minecart = 10,
    ActivatedTnt = 50,
    EnderCrystal = 51,
    Arrow = 60,
    Snowball = 61,
    Egg = 62,
    Fireball = 63,
    FireCharge = 64,
    EnderPearl = 65,
    WitherSkull = 66,
    FallingObjects = 70,
    ItemFrame = 71,
    EyeOfEnder = 72,
    Potion = 73,
    FallingDragonEgg = 74,
    ExpBottle = 75,
    FireworkRocket = 76,
    LeashKnot = 77,
    ArmorStand = 78,
    FishingFloat = 90,
}
