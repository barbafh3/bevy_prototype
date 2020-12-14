use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use super::BuilderStates;
use crate::{
    characters::{builder::Builder, villager_idle_move, VillagerMovement},
    managers::villagers::IdleVillager,
};

pub fn state_builder_idle(
    delta: f32,
    commands: &mut Commands,
    entity: Entity,
    builder: &mut Mut<Builder>,
    movement: &mut VillagerMovement,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    villager_idle_move(movement, delta, transform, rb_set, rb_handle);
    if !builder.is_idle {
        commands.insert_one(entity, IdleVillager);
        builder.is_idle = true;
    }
    if !builder.requested_construction.is_none() {
        builder.is_idle = false;
        commands.remove_one::<IdleVillager>(entity);
        println!("BuilderIdle: Builder is now working");
        builder.state = BuilderStates::Working;
    }
}
