pub mod states;

use bevy::{ecs::Query, ecs::Res, input::Input, prelude::KeyCode};

use crate::entities::player::Player;

use states::PlayerStates;

// use super::states::PlayerStates;

pub fn sys_player_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        if keyboard_input.just_released(KeyCode::C) {
            player.state = PlayerStates::Active;
            println!("Player state changed to Active");
        }
        if keyboard_input.just_released(KeyCode::I) {
            player.state = PlayerStates::Idle;
            println!("Player state changed to Idle");
        }
        if keyboard_input.just_released(KeyCode::K) {
            player.state = PlayerStates::Knight;
            println!("Player state changed to Knight");
        }
    }
}
