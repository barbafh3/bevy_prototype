use crate::characters::{hauler::Hauler, IdleMovement};
use bevy::{
    ecs::{Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

pub fn state_hauler_idle(
    delta: f32,
    hauler: &mut Mut<Hauler>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    hauler.idle_move(delta, transform, rb_set, rb_handle);
}
