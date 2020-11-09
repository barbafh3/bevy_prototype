use bevy::math::Vec3;

use crate::systems::player::states::PlayerStates;

pub struct Player {
    pub state: PlayerStates,
    pub speed: f32,
    pub base_movement_tick: f32,
    pub movement_tick: f32,
    pub movement_radius: f32,
    pub movement_target: Vec3,
}
