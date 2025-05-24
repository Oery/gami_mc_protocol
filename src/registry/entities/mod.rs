use std::fmt::Debug;

use player::Player;

use crate::packets::play::server::{SpawnMob, SpawnObject, SpawnPlayer};
use crate::registry::entities::mobs::*;
use crate::registry::entities::objects::*;
use crate::serialization::Metadata;

pub mod metadata;
pub mod mobs;
pub mod objects;
pub mod player;

pub trait Entity: Debug {
    fn update(&mut self, metadatas: &[Metadata]);
    fn id(&self) -> i32;
    fn is_on_fire(&self) -> bool;
    fn is_crouching(&self) -> bool;
    fn is_sprinting(&self) -> bool;
    fn is_eating_drinking_blocking(&self) -> bool;
    fn is_invisible(&self) -> bool;
    fn air(&self) -> i16;
    fn name_tag(&self) -> &str;
    fn always_show_name_tag(&self) -> bool;
    fn is_silent(&self) -> bool;
}

pub trait LivingEntity: Entity {
    fn ai_disabled(&self) -> bool;
    fn update(&mut self, metadatas: &[Metadata]);
}

pub trait Ageable: LivingEntity {
    fn age(&self) -> u8;
    fn update(&mut self, metadatas: &[Metadata]);
}

pub trait Tameable: Ageable {
    fn owner_name(&self) -> &str;
    fn is_sitting(&self) -> bool;
    fn is_tame(&self) -> bool;
    fn update(&mut self, metadatas: &[Metadata]);
}

pub trait Object<'a>: From<&'a SpawnObject> + Entity {
    fn update(&mut self, metadatas: &[Metadata]);
}

#[derive(Debug, PartialEq)]
pub enum EntityKind {
    // Objects
    Boat(Boat),
    ItemStack(ItemStack),
    AreaEffectCloud(AreaEffectCloud),
    Minecart(Minecart),
    ActivatedTnt(ActivatedTnt),
    EnderCrystal(EnderCrystal),
    Arrow(Arrow),
    Snowball(Snowball),
    Egg(Egg),
    Fireball(Fireball),
    FireCharge(FireCharge),
    EnderPearl(EnderPearl),
    WitherSkull(WitherSkull),
    FallingObjects(FallingObjects),
    ItemFrame(ItemFrame),
    EyeOfEnder(EyeOfEnder),
    Potion(Potion),
    FallingDragonEgg(FallingDragonEgg),
    ExpBottle(ExpBottle),
    FireworkRocket(FireworkRocket),
    LeashKnot(LeashKnot),
    ArmorStand(ArmorStand),
    FishingFloat(FishingFloat),
    ExperienceOrb(ExperienceOrb),

    // Monsters
    Creeper(Creeper),
    Skeleton(Skeleton),
    Spider(Spider),
    Giant(Giant),
    Zombie(Zombie),
    Slime(Slime),
    Ghast(Ghast),
    ZombiePigman(ZombiePigman),
    Enderman(Enderman),
    CaveSpider(CaveSpider),
    Silverfish(Silverfish),
    Blaze(Blaze),
    MagmaCube(MagmaCube),
    EnderDragon(EnderDragon),
    WitherBoss(WitherBoss),
    Bat(Bat),
    Witch(Witch),
    Endermite(Endermite),
    Guardian(Guardian),

    // Animals
    Pig(Pig),
    Sheep(Sheep),
    Cow(Cow),
    Chicken(Chicken),
    Squid(Squid),
    Wolf(Wolf),
    Mooshroom(Mooshroom),
    SnowGolem(SnowGolem),
    Ocelot(Ocelot),
    IronGolem(IronGolem),
    Horse(Horse),
    Rabbit(Rabbit),
    Villager(Villager),

    // Players
    Player(Player),
}

impl From<&SpawnPlayer> for EntityKind {
    fn from(player: &SpawnPlayer) -> Self {
        Self::Player(player.into())
    }
}

impl From<&SpawnObject> for EntityKind {
    fn from(object: &SpawnObject) -> Self {
        match object.kind {
            ObjectKind::Boat => Self::Boat(object.into()),
            ObjectKind::ItemStack => Self::ItemStack(object.into()),
            ObjectKind::AreaEffectCloud => Self::AreaEffectCloud(object.into()),
            ObjectKind::Minecart => Self::Minecart(object.into()),
            ObjectKind::ActivatedTnt => Self::ActivatedTnt(object.into()),
            ObjectKind::EnderCrystal => Self::EnderCrystal(object.into()),
            ObjectKind::Arrow => Self::Arrow(object.into()),
            ObjectKind::Snowball => Self::Snowball(object.into()),
            ObjectKind::Egg => Self::Egg(object.into()),
            ObjectKind::Fireball => Self::Fireball(object.into()),
            ObjectKind::FireCharge => Self::FireCharge(object.into()),
            ObjectKind::EnderPearl => Self::EnderPearl(object.into()),
            ObjectKind::WitherSkull => Self::WitherSkull(object.into()),
            ObjectKind::FallingObjects => Self::FallingObjects(object.into()),
            ObjectKind::ItemFrame => Self::ItemFrame(object.into()),
            ObjectKind::EyeOfEnder => Self::EyeOfEnder(object.into()),
            ObjectKind::Potion => Self::Potion(object.into()),
            ObjectKind::FallingDragonEgg => Self::FallingDragonEgg(object.into()),
            ObjectKind::ExpBottle => Self::ExpBottle(object.into()),
            ObjectKind::FireworkRocket => Self::FireworkRocket(object.into()),
            ObjectKind::LeashKnot => Self::LeashKnot(object.into()),
            ObjectKind::ArmorStand => Self::ArmorStand(object.into()),
            ObjectKind::FishingFloat => Self::FishingFloat(object.into()),
        }
    }
}

impl From<&SpawnMob> for EntityKind {
    fn from(mob: &SpawnMob) -> Self {
        match mob.kind {
            // Monsters
            Mob::Creeper => Self::Creeper(mob.into()),
            Mob::Skeleton => Self::Skeleton(mob.into()),
            Mob::Spider => Self::Spider(mob.into()),
            Mob::Giant => Self::Giant(mob.into()),
            Mob::Zombie => Self::Zombie(mob.into()),
            Mob::Slime => Self::Slime(mob.into()),
            Mob::Ghast => Self::Ghast(mob.into()),
            Mob::ZombiePigman => Self::ZombiePigman(mob.into()),
            Mob::Enderman => Self::Enderman(mob.into()),
            Mob::CaveSpider => Self::CaveSpider(mob.into()),
            Mob::Silverfish => Self::Silverfish(mob.into()),
            Mob::Blaze => Self::Blaze(mob.into()),
            Mob::MagmaCube => Self::MagmaCube(mob.into()),
            Mob::EnderDragon => Self::EnderDragon(mob.into()),
            Mob::WitherBoss => Self::WitherBoss(mob.into()),
            Mob::Bat => Self::Bat(mob.into()),
            Mob::Witch => Self::Witch(mob.into()),
            Mob::Endermite => Self::Endermite(mob.into()),
            Mob::Guardian => Self::Guardian(mob.into()),

            // Animals
            Mob::Pig => Self::Pig(mob.into()),
            Mob::Sheep => Self::Sheep(mob.into()),
            Mob::Cow => Self::Cow(mob.into()),
            Mob::Chicken => Self::Chicken(mob.into()),
            Mob::Squid => Self::Squid(mob.into()),
            Mob::Wolf => Self::Wolf(mob.into()),
            Mob::Mooshroom => Self::Mooshroom(mob.into()),
            Mob::SnowGolem => Self::SnowGolem(mob.into()),
            Mob::Ocelot => Self::Ocelot(mob.into()),
            Mob::IronGolem => Self::IronGolem(mob.into()),
            Mob::Horse => Self::Horse(mob.into()),
            Mob::Rabbit => Self::Rabbit(mob.into()),
            Mob::Villager => Self::Villager(mob.into()),
        }
    }
}

impl EntityKind {
    pub fn update(&mut self, metadatas: &[Metadata]) {
        if let Some(entity) = self.as_entity_mut() {
            entity.update(metadatas);
        }
    }

    pub fn id(&self) -> i32 {
        self.as_entity().id()
    }

    pub fn as_entity(&self) -> &dyn Entity {
        use EntityKind::*;

        match self {
            Creeper(ref entity) => entity,
            Skeleton(ref skeleton) => skeleton,
            Spider(ref spider) => spider,
            Boat(ref boat) => boat,
            ItemStack(ref item) => item,
            Minecart(ref minecart) => minecart,
            EnderCrystal(ref crystal) => crystal,
            Arrow(ref arrow) => arrow,
            ItemFrame(ref frame) => frame,
            FireworkRocket(ref rocket) => rocket,
            LeashKnot(ref knot) => knot,
            ArmorStand(ref armor_stand) => armor_stand,
            Giant(ref giant) => giant,
            Zombie(ref zombie) => zombie,
            Slime(ref slime) => slime,
            Ghast(ref ghast) => ghast,
            ZombiePigman(ref zombie_pigman) => zombie_pigman,
            Enderman(ref enderman) => enderman,
            CaveSpider(ref cave_spider) => cave_spider,
            Silverfish(ref silverfish) => silverfish,
            Blaze(ref blaze) => blaze,
            MagmaCube(ref magma_cube) => magma_cube,
            EnderDragon(ref ender_dragon) => ender_dragon,
            WitherBoss(ref wither_boss) => wither_boss,
            Bat(ref bat) => bat,
            Witch(ref witch) => witch,
            Endermite(ref endermite) => endermite,
            Guardian(ref guardian) => guardian,
            Pig(ref pig) => pig,
            Sheep(ref sheep) => sheep,
            Cow(ref cow) => cow,
            Chicken(ref chicken) => chicken,
            Squid(ref squid) => squid,
            Wolf(ref wolf) => wolf,
            Mooshroom(ref mooshroom) => mooshroom,
            SnowGolem(ref snow_golem) => snow_golem,
            Ocelot(ref ocelot) => ocelot,
            IronGolem(ref iron_golem) => iron_golem,
            Horse(ref horse) => horse,
            Rabbit(ref rabbit) => rabbit,
            Villager(ref villager) => villager,
            AreaEffectCloud(ref area_effect_cloud) => area_effect_cloud,
            ActivatedTnt(ref activated_tnt) => activated_tnt,
            Snowball(ref snowball) => snowball,
            Egg(ref egg) => egg,
            Fireball(ref fireball) => fireball,
            FireCharge(ref charge) => charge,
            EnderPearl(ref pearl) => pearl,
            WitherSkull(ref skull) => skull,
            FallingObjects(ref falling) => falling,
            EyeOfEnder(ref eye) => eye,
            Potion(ref potion) => potion,
            FallingDragonEgg(ref falling) => falling,
            ExpBottle(ref bottle) => bottle,
            FishingFloat(ref float) => float,
            ExperienceOrb(ref orb) => orb,
            Player(ref player) => player,
        }
    }

    pub(crate) fn as_entity_mut(&mut self) -> Option<&mut dyn Entity> {
        use EntityKind::*;

        match self {
            Creeper(ref mut entity) => Some(entity),
            Skeleton(ref mut entity) => Some(entity),
            Spider(ref mut entity) => Some(entity),
            Boat(ref mut boat) => Some(boat),
            ItemStack(ref mut item_stack) => Some(item_stack),
            Minecart(ref mut minecart) => Some(minecart),
            EnderCrystal(ref mut ender_crystal) => Some(ender_crystal),
            Arrow(ref mut arrow) => Some(arrow),
            ItemFrame(ref mut item_frame) => Some(item_frame),
            FireworkRocket(ref mut firework_rocket) => Some(firework_rocket),
            LeashKnot(ref mut leash_knot) => Some(leash_knot),
            ArmorStand(ref mut armor_stand) => Some(armor_stand),
            Giant(ref mut giant) => Some(giant),
            Zombie(ref mut zombie) => Some(zombie),
            Slime(ref mut slime) => Some(slime),
            Ghast(ref mut ghast) => Some(ghast),
            ZombiePigman(ref mut zombie_pigman) => Some(zombie_pigman),
            Enderman(ref mut enderman) => Some(enderman),
            CaveSpider(ref mut cave_spider) => Some(cave_spider),
            Silverfish(ref mut silverfish) => Some(silverfish),
            Blaze(ref mut blaze) => Some(blaze),
            MagmaCube(ref mut magma_cube) => Some(magma_cube),
            EnderDragon(ref mut ender_dragon) => Some(ender_dragon),
            WitherBoss(ref mut wither_boss) => Some(wither_boss),
            Bat(ref mut bat) => Some(bat),
            Witch(ref mut witch) => Some(witch),
            Endermite(ref mut endermite) => Some(endermite),
            Guardian(ref mut guardian) => Some(guardian),
            Pig(ref mut pig) => Some(pig),
            Sheep(ref mut sheep) => Some(sheep),
            Cow(ref mut cow) => Some(cow),
            Chicken(ref mut chicken) => Some(chicken),
            Squid(ref mut squid) => Some(squid),
            Wolf(ref mut wolf) => Some(wolf),
            Mooshroom(ref mut mooshroom) => Some(mooshroom),
            SnowGolem(ref mut snow_golem) => Some(snow_golem),
            Ocelot(ref mut ocelot) => Some(ocelot),
            IronGolem(ref mut iron_golem) => Some(iron_golem),
            Horse(ref mut horse) => Some(horse),
            Rabbit(ref mut rabbit) => Some(rabbit),
            Villager(ref mut villager) => Some(villager),
            AreaEffectCloud(ref mut e) => Some(e),
            ActivatedTnt(ref mut e) => Some(e),
            Snowball(ref mut e) => Some(e),
            Egg(ref mut e) => Some(e),
            Fireball(ref mut e) => Some(e),
            FireCharge(ref mut e) => Some(e),
            EnderPearl(ref mut e) => Some(e),
            WitherSkull(ref mut e) => Some(e),
            FallingObjects(ref mut e) => Some(e),
            EyeOfEnder(ref mut e) => Some(e),
            Potion(ref mut e) => Some(e),
            FallingDragonEgg(ref mut e) => Some(e),
            ExpBottle(ref mut e) => Some(e),
            FishingFloat(ref mut e) => Some(e),
            ExperienceOrb(ref mut e) => Some(e),
            Player(ref mut player) => Some(player),
        }
    }
}
