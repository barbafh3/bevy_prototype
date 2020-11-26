use strum_macros::*;

#[derive(Debug, Display, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameResources {
    Wood,
    Stone,
    Plank,
    StoneBrick,
}

#[derive(Debug, Display, Clone, Copy, Eq, PartialEq)]
pub enum Jobs {
    Villager,
    Hauler,
    Builder,
    Carpenter,
    Woodcutter,
}

#[derive(Debug, Display)]
pub enum Tasks {
    Haul,
    Build,
}
