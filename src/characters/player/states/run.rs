use bevy::{
    ecs::Mut,
    ecs::Res,
    ecs::ResMut,
    input::Input,
    math::{Quat, Vec3},
    prelude::{KeyCode, Transform},
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::characters::player::Player;

pub fn state_player_run(
    keyboard_input: &Res<Input<KeyCode>>,
    rb_set: &mut ResMut<RigidBodySet>,
    mut player: Mut<Player>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    mut transform: Mut<Transform>,
) {
    let rb_index = rb_handle.handle();
    let mut rb = rb_set.get_mut(rb_index).unwrap();
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
    if direction.x() == 0.0 && direction.y() == 0.0 {
        println!("Player state changed to Idle");
        rb.linvel = Vector2::new(0.0, rb.linvel.y);
        player.state = super::PlayerStates::Idle;
    } else {
        rb.linvel = Vector2::new(direction.x() * player.speed, rb.linvel.y);
        // transform.rotation = Quat::from_rotation_y(0.0);
    }
}
