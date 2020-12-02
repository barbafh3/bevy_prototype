pub mod states;

use bevy::ecs::Entity;
use states::BuilderStates;

use super::VillagerMovement;

pub struct Builder {
    pub state: BuilderStates,
    pub movement: VillagerMovement,
    pub requested_construction: Entity,
    pub current_construction: Entity,
    pub is_inside_building: bool,
}
