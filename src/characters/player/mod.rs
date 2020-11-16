pub mod states;

use bevy::{ecs::Query, ecs::Res, input::Input, math::Vec3, prelude::KeyCode};

use states::PlayerStates;

pub struct Player {
    pub state: PlayerStates,
    pub speed: f32,
    pub base_movement_tick: f32,
    pub movement_tick: f32,
    pub movement_radius: f32,
    pub movement_target: Vec3,
}

pub fn sys_player_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        // if keyboard_input.just_released(KeyCode::C) {
        //     player.state = PlayerStates::Run;
        //     println!("Player state changed to Active");
        // }
        // if keyboard_input.just_released(KeyCode::I) {
        //     player.state = PlayerStates::Idle;
        //     println!("Player state changed to Idle");
        // }
        // if keyboard_input.just_released(KeyCode::Space) {
        //     player.state = PlayerStates::Jump;
        //     println!("Player jump!");
        // }
        // if keyboard_input.just_released(KeyCode::K) {
        //     player.state = PlayerStates::Knight;
        //     println!("Player state changed to Knight");
        // }
    }
}
