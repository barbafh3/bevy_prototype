use crate::{
    characters::hauler::{hauler_idle_move, Hauler},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use super::HaulerStates;

pub fn state_hauler_idle(
    delta: f32,
    commands: &mut Commands,
    entity: Entity,
    hauler: &mut Mut<Hauler>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    hauler_idle_move(hauler, delta, transform, rb_set, rb_handle);
    if !hauler.is_idle {
        commands.insert_one(entity, IdleVillager);
        hauler.is_idle = true;
    }
    if hauler.amount_requested > 0 {
        hauler.state = HaulerStates::Loading;
    }
}
