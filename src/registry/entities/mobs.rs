use crate::serialization::{Deserialize, Serialize};
use gami_macros::{Deserialize, Serialize};

mod bat;
pub use bat::Bat;

mod blaze;
pub use blaze::Blaze;

mod cave_spider;
pub use cave_spider::CaveSpider;

mod chicken;
pub use chicken::Chicken;

mod cow;
pub use cow::Cow;

mod creeper;
pub use creeper::Creeper;

mod enderman;
pub use enderman::Enderman;

mod ghast;
pub use ghast::Ghast;

mod guardian;
pub use guardian::Guardian;

mod horse;
pub use horse::Horse;

mod iron_golem;
pub use iron_golem::IronGolem;

mod magma_cube;
pub use magma_cube::MagmaCube;

mod mooshroom;
pub use mooshroom::Mooshroom;

mod pig;
pub use pig::Pig;

mod rabbit;
pub use rabbit::Rabbit;

mod sheep;
pub use sheep::{Sheep, SheepColor};

mod skeleton;
pub use skeleton::Skeleton;

mod slime;
pub use slime::Slime;

mod spider;
pub use spider::Spider;

mod villager;
pub use villager::Villager;

mod witch;
pub use witch::Witch;

mod zombie;
pub use zombie::Zombie;

mod zombie_pigman;
pub use zombie_pigman::ZombiePigman;

mod ocelot;
pub use ocelot::Ocelot;

mod wolf;
pub use wolf::Wolf;

mod wither_boss;
pub use wither_boss::WitherBoss;

mod endermite;
pub use endermite::Endermite;

mod ender_dragon;
pub use ender_dragon::EnderDragon;

mod silverfish;
pub use silverfish::Silverfish;

mod snow_golem;
pub use snow_golem::SnowGolem;

mod squid;
pub use squid::Squid;

mod giant;
pub use giant::Giant;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[encoding("u8")]
pub enum Mob {
    // Monsters
    Creeper = 50,
    Skeleton = 51,
    Spider = 52,
    Giant = 53,
    Zombie = 54,
    Slime = 55,
    Ghast = 56,
    ZombiePigman = 57,
    Enderman = 58,
    CaveSpider = 59,
    Silverfish = 60,
    Blaze = 61,
    MagmaCube = 62,
    EnderDragon = 63,
    WitherBoss = 64,
    Bat = 65,
    Witch = 66,
    Endermite = 67,
    Guardian = 68,

    // Animals
    Pig = 90,
    Sheep = 91,
    Cow = 92,
    Chicken = 93,
    Squid = 94,
    Wolf = 95,
    Mooshroom = 96,
    SnowGolem = 97,
    Ocelot = 98,
    IronGolem = 99,
    Horse = 100,
    Rabbit = 101,
    Villager = 120,
}
