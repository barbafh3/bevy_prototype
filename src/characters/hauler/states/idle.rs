use crate::{
    characters::{hauler::Hauler, IdleMovement},
    get_idle_point,
};
use bevy::{
    ecs::{Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use rand::prelude::*;

pub fn state_hauler_idle(
    delta: f32,
    hauler: &mut Mut<Hauler>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    hauler.idle_move(delta, transform, rb_set, rb_handle);
}
