use crate::characters::{hauler::Hauler, normalize};
use bevy::{
    ecs::{Mut, Query, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

pub fn state_hauler_carrying(
    hauler: &mut Hauler,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    destination_query: &Query<&Transform>,
) {
    let rb_index = rb_handle.handle();
    let mut rb = rb_set.get_mut(rb_index).unwrap();
    let target_transform = destination_query
        .get(hauler.resource_destination.unwrap())
        .unwrap();
    if hauler.capacity > 0 {
        let vector = target_transform.translation - transform.translation;
        let direction = normalize(vector);
        rb.linvel = direction * hauler.speed;
    } else {
        hauler.resource_destination = None;
        hauler.amount_requested = 0;
        println!("Hauler: State changed to Idle");
        hauler.state = super::HaulerStates::Idle;
    }
}
