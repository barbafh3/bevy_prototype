use strum_macros::*;

#[derive(Debug, Display, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum GameResources {
    Wood,
    Stone,
    Plank,
    StoneBrick,
}

#[derive(Debug, Display)]
pub enum Jobs {
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
