pub mod states;

use super::VillagerMovement;
use bevy::ecs::Entity;
use states::BuilderStates;

pub struct Builder {
    pub state: BuilderStates,
    pub movement: VillagerMovement,
    pub requested_construction: Entity,
    pub current_construction: Entity,
    pub is_inside_building: bool,
}
