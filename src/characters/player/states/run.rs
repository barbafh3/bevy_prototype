use bevy::{ecs::Mut, ecs::Res, ecs::ResMut, input::Input, prelude::KeyCode};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::characters::player::{get_input_direction, Player};

use super::PlayerStates;

pub fn state_player_run(
    keyboard_input: &Res<Input<KeyCode>>,
    rb_set: &mut ResMut<RigidBodySet>,
    mut player: Mut<Player>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let mut rb = rb_set.get_mut(rb_index).unwrap();
    let direction = get_input_direction(keyboard_input);
    if direction.x() == 0.0 && direction.y() == 0.0 {
        // rb.linvel = Vector2::new(0.0, rb.linvel.y);
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
        player.state = PlayerStates::Idle;
    } else {
        // rb.linvel = Vector2::new(direction.x() * player.speed, rb.linvel.y);
        rb.set_linvel(
            Vector2::new(direction.x() * player.speed, direction.y() * player.speed),
            true,
        );
    }
}
