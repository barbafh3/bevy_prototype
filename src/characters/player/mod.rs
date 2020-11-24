pub mod states;

use bevy::{ecs::Query, ecs::Res, input::Input, math::Vec3, prelude::KeyCode};

use bevy_rapier2d::rapier::geometry::Proximity;
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

impl Player {
    pub fn on_proximity_event(&self, entering: Proximity) -> String {
        let output = "".to_string();
        // let mut output = "Player ".to_string();
        // match entering {
        //     Proximity::Disjoint => output.push_str("just left a sensor"),
        //     Proximity::Intersecting => output.push_str("entered a sensor"),
        //     _ => (),
        // }
        return output;
    }
}

pub fn get_input_direction(keyboard_input: &Res<Input<KeyCode>>) -> Vec3 {
    let mut direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Left) {
        *direction.x_mut() = direction.x() - 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        *direction.x_mut() = direction.x() + 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        *direction.y_mut() = direction.y() - 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        *direction.y_mut() = direction.y() + 1.0;
    }
    return direction;
}
