use bevy::{
    ecs::{Commands, Entity, Mut, Query, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{characters::hauler::Hauler, characters::normalize, managers::villagers::IdleVillager};

pub fn state_hauler_loading(
    entity: Entity,
    commands: &mut Commands,
    hauler: &mut Hauler,
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
    println!("Hauler: {:?}", &hauler);
    if hauler.capacity <= 0 {
        let vector = target_transform.translation - transform.translation;
        let direction = normalize(vector);
        rb.set_linvel(direction * hauler.speed, true);
    } else {
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
        hauler.resource_origin = None;
        println!("Hauler: State changed to Carrying");
        hauler.state = super::HaulerStates::Carrying;
    }
}
