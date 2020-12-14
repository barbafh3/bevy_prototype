use crate::{
    characters::{hauler::Hauler, VillagerMovement},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, Query, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

pub fn state_hauler_loading(
    commands: &mut Commands,
    entity: Entity,
    hauler: &mut Hauler,
    movement: &mut VillagerMovement,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    origin_query: &Query<&Transform>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();
    if hauler.is_idle {
        commands.remove_one::<IdleVillager>(entity);
        hauler.is_idle = false;
    }
    let target_transform = origin_query.get(hauler.resource_origin.unwrap()).unwrap();
    if hauler.capacity <= 0 {
        let vector = target_transform.translation - transform.translation;
        let is_far_enough = vector.x().abs() > 2.0 || vector.y().abs() > 2.0;
        if is_far_enough {
            let target_vector = Vector2::new(vector.x(), vector.y());
            let direction = target_vector.normalize();
            let distance: Vector2<f32> = direction * movement.speed;
            rb.set_linvel(distance, true);
        } else {
            println!("HaulerLoading: Arrived and waiting for cargo");
        }
    } else {
        println!("HaulerLoading: Cargo loaded");
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
        hauler.resource_origin = None;
        hauler.state = super::HaulerStates::Carrying;
    }
}
