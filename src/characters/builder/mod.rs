pub mod states;

use bevy::ecs::{Entity, Mut};
use states::BuilderStates;

use super::VillagerMovement;

pub struct Builder {
    pub state: BuilderStates,
    pub construction_tick: f32,
    pub movement: VillagerMovement,
    pub requested_construction: Option<Entity>,
    pub current_construction: Option<Entity>,
    pub is_inside_building: bool,
}

impl Builder {
    pub fn new(
        construction_tick: f32,
        speed: f32,
        base_movement_tick: f32,
        movement_radius: f32,
    ) -> Builder {
        Builder {
            state: BuilderStates::Idle,
            construction_tick,
            movement: VillagerMovement {
                base_tick: base_movement_tick,
                tick: base_movement_tick,
                speed,
                radius: movement_radius,
            },
            requested_construction: None,
            current_construction: None,
            is_inside_building: false,
        }
    }
}
