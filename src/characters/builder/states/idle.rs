use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{
    characters::{builder::Builder, IdleMovement},
    managers::villagers::IdleVillager,
};

use super::BuilderStates;

pub fn state_builder_idle(
    delta: f32,
    commands: &mut Commands,
    entity: Entity,
    builder: &mut Mut<Builder>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    builder.idle_move(delta, transform, rb_set, rb_handle);
    if !builder.is_idle {
        commands.insert_one(entity, IdleVillager);
        builder.is_idle = true;
    }
    if !builder.requested_construction.is_none() {
        builder.state = BuilderStates::Working;
    }
}
