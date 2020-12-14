use enum_map::{enum_map, Enum, EnumMap};
use strum_macros::*;

#[allow(dead_code)]
#[derive(Debug, Display, Hash, Eq, PartialEq, Clone, Copy, Enum)]
pub enum GameResources {
    Wood,
    Stone,
    Plank,
    StoneBrick,
}

#[allow(dead_code)]
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq)]
pub enum Jobs {
    Villager,
    Hauler,
    Builder,
    Carpenter,
    Woodcutter,
}

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum Tasks {
    Haul,
    Build,
}

pub fn get_resources_list() -> EnumMap<GameResources, i32> {
    return enum_map! {
        GameResources::Wood => 0,
        GameResources::Stone => 0,
        GameResources::Plank => 0,
        GameResources::StoneBrick => 0,
    };
}
