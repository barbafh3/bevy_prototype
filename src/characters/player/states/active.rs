use bevy::{
    ecs::Mut,
    ecs::Res,
    input::Input,
    math::Vec3,
    prelude::{KeyCode, Transform},
};
use bevy_rapier2d::{
    na::Vector2,
    rapier::{dynamics::RigidBody, ncollide::math::Isometry},
};

use crate::characters::player::Player;

pub fn player_active_state(
    delta: f32,
    keyboard_input: &Res<Input<KeyCode>>,
    player: Mut<Player>,
    mut transform: Mut<Transform>,
) {
    let translation = &mut transform.translation;
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
    *translation += direction * player.speed * delta;
    // let vector = Vector2::new(translation.x(), translation.y());
    // rigid_body.set_position(Isometry::new(vector, 0.0));
}
