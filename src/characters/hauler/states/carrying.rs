use crate::characters::{hauler::Hauler, VillagerMovement};
use bevy::{
    ecs::{Mut, Query, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

pub fn state_hauler_carrying(
    hauler: &mut Hauler,
    movement: &mut VillagerMovement,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    destination_query: &Query<&Transform>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();
    let target_transform = destination_query
        .get(hauler.resource_destination.unwrap())
        .unwrap();
    if hauler.capacity > 0 {
        let vector = target_transform.translation - transform.translation;
        let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
        if is_far_enough {
            let target_vector = Vector2::new(vector.x(), vector.y());
            let direction = target_vector.normalize();
            rb.set_linvel(direction * movement.speed, true);
        }
    } else {
        hauler.resource_destination = None;
        hauler.amount_requested = 0;
        hauler.state = super::HaulerStates::Finished;
        println!("Hauler finished working");
    }
}
