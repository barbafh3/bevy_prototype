pub mod active;
pub mod idle;
pub mod knight;

use bevy::{
    core::Time,
    // ecs::Mut,
    ecs::ResMut,
    ecs::{Query, Res},
    input::Input,
    prelude::AssetServer,
    prelude::Assets,
    prelude::Handle,
    prelude::{KeyCode, Transform},
    sprite::ColorMaterial,
};

use crate::entities::player::Player;

use self::{active::player_active_state, idle::player_idle_state};

#[derive(Eq, PartialEq)]
pub enum PlayerStates {
    Idle,
    Active,
    Knight,
}

pub fn run_player_state(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Handle<ColorMaterial>)>,
) {
    for (player, transform, _) in query.iter_mut() {
        match player.state {
            PlayerStates::Idle => player_idle_state(time.delta_seconds, player, transform),
            PlayerStates::Active => {
                player_active_state(time.delta_seconds, &keyboard_input, player, transform)
            }
            _ => (),
        }
    }
    for (player, _, mut material) in query.iter_mut() {
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
