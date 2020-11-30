pub mod idle;
pub mod jump;
pub mod knight;
pub mod run;

use bevy::{
    core::Time,
    ecs::ResMut,
    ecs::{Query, Res},
    input::Input,
    prelude::AssetServer,
    prelude::Assets,
    prelude::Handle,
    prelude::{KeyCode, Transform},
    sprite::ColorMaterial,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use self::{idle::state_player_idle, jump::state_player_jump, run::state_player_run};

use super::Player;

#[derive(Eq, PartialEq)]
pub enum PlayerStates {
    Idle,
    Run,
    Knight,
    Jump,
}

pub fn player_state_transitions(
    keyboard_input: Res<Input<KeyCode>>,
    rigidbody_set: ResMut<RigidBodySet>,
    mut query: Query<(&mut Player, &mut RigidBodyHandleComponent)>,
) {
    for (mut player, rigidbody_handle) in query.iter_mut() {
        let rigidbody_index = rigidbody_handle.handle();
        let rigidbody = rigidbody_set.get(rigidbody_index).unwrap();
        if rigidbody.linvel().y < 0.0 {
            println!("Player falling!!!");
        } else if rigidbody.linvel().x == 0.0 && rigidbody.linvel().y == 0.0 {
            println!("Player state changed to Idle");
            player.state = PlayerStates::Idle;
        } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D) {
            println!("Player state changed to Run");
            player.state = PlayerStates::Run;
        }
    }
}

pub fn run_player_state(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &mut Handle<ColorMaterial>,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (player, transform, _, rb_handle) in query.iter_mut() {
        match player.state {
            PlayerStates::Idle => {
                state_player_idle(time.delta_seconds, &keyboard_input, player, transform)
            }
            PlayerStates::Run => state_player_run(&keyboard_input, &mut rb_set, player, rb_handle),
            PlayerStates::Jump => state_player_jump(),
            _ => (),
        }
    }
    for (player, _, mut material, _) in query.iter_mut() {
        match player.state {
            PlayerStates::Knight => {
                let texture_handle = asset_server.load("knight.png");
                *material = materials.add(texture_handle.into());
            }
            _ => {
                let texture_handle = asset_server.load("archer.png");
                *material = materials.add(texture_handle.into());
            }
        }
    }
}
