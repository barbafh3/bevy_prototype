use bevy::{
    ecs::{Mut, Res},
    input::Input,
    math::Vec3,
    prelude::KeyCode,
    prelude::Transform,
};
use rand::prelude::*;

use crate::{characters::player::Player, get_idle_point};

use super::PlayerStates;

pub fn state_player_idle(
    delta: f32,
    keyboard_input: &Res<Input<KeyCode>>,
    mut player: Mut<Player>,
    mut transform: Mut<Transform>,
) {
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) {
        player.state = PlayerStates::Run;
    }

    // let translation = &mut transform.translation;
    // player.movement_tick = run_movement_tick(&player, delta);
    // let can_change_target = player.movement_tick <= 0.0;
    // if can_change_target {
    //     player.movement_target = get_new_position(
    //         get_idle_point().x(),
    //         get_idle_point().y(),
    //         player.movement_radius.clone(),
    //     );
    //     player.movement_tick = player.base_movement_tick.clone();
    // }
    // let target_vector = player.movement_target - translation.clone();
    // let is_far_enough = target_vector.x().abs() > 2.0 && target_vector.y().abs() > 2.0;
    // if is_far_enough {
    //     let direction = normalize(target_vector);
    //     *translation += direction * player.speed * delta;
    // }
}

fn normalize(position: Vec3) -> Vec3 {
    let mut direction_x: f32 = 0.0;
    let mut direction_y: f32 = 0.0;
    if position.x() > 0.0 {
        direction_x = 1.0;
    }
    if position.x() < 0.0 {
        direction_x = -1.0;
    }
    if position.y() > 0.0 {
        direction_y = 1.0;
    }
    if position.y() < 0.0 {
        direction_y = -1.0;
    }
    return Vec3::new(direction_x, direction_y, 0.0);
}

fn get_new_position(x: f32, y: f32, radius: f32) -> Vec3 {
    let random_x = thread_rng().gen_range(x - radius, x + radius);
    let random_y = thread_rng().gen_range(y - radius, y + radius);
    Vec3::new(random_x, random_y, 0.0)
}

fn run_movement_tick(player: &Mut<Player>, delta: f32) -> f32 {
    if player.movement_tick > 0.0 {
        return player.movement_tick - delta;
    } else {
        return player.movement_tick;
    }
}
