use crate::{
    characters::{hauler::Hauler, VillagerMovement},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use super::HaulerStates;

pub fn state_hauler_finished_work(
    commands: &mut Commands,
    entity: Entity,
    hauler: &mut Mut<Hauler>,
    movement: &mut VillagerMovement,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();

    if !hauler.is_idle {
        commands.insert_one(entity, IdleVillager);
        hauler.is_idle = true;
    }

    if hauler.amount_requested > 0 {
        hauler.state = HaulerStates::Loading;
    }

    let vector = movement.target - transform.translation;
    let is_far_enough = vector.x.abs() > 2.0 && vector.y.abs() > 2.0;
    if is_far_enough {
        let target_vector = Vector2::new(vector.x, vector.y);
        let direction = target_vector.normalize();
        rb.set_linvel(direction * movement.speed, true);
    } else {
        println!("Hauler is now idle");
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
        hauler.state = HaulerStates::Idle;
    }
}
