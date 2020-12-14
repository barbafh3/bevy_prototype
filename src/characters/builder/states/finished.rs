use crate::{characters::builder::Builder, get_idle_point, managers::villagers::IdleVillager};
use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use super::BuilderStates;

pub fn state_builder_finished_work(
    commands: &mut Commands,
    entity: Entity,
    builder: &mut Mut<Builder>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();

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

    let vector = get_idle_point() - transform.translation;
    let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
    if is_far_enough {
        let target_vector = Vector2::new(vector.x(), vector.y());
        let direction = target_vector.normalize();
        rb.set_linvel(direction * builder.movement.speed, true);
    } else {
        println!("Hauler is now idle");
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
        builder.state = BuilderStates::Idle;
    }
}
