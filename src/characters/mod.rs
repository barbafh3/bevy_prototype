pub mod builder;
pub mod hauler;
pub mod villager;

use bevy::{
    ecs::{Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use rand::*;

use crate::get_idle_point;

#[derive(Debug, PartialEq)]
pub struct VillagerMovement {
    pub speed: f32,
    pub base_tick: f32,
    pub tick: f32,
    pub radius: f32,
    pub target: Vec3,
}

pub fn villager_idle_move(
    movement: &mut VillagerMovement,
    delta: f32,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();
    movement.tick = run_movement_tick(movement, delta);
    let can_change_target = movement.tick <= 0.0;
    if can_change_target {
        movement.target = get_new_position(
            get_idle_point().x(),
            get_idle_point().y(),
            movement.radius.clone(),
        );
        movement.tick = movement.base_tick.clone();
    }
    let vector = movement.target - transform.translation;
    let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
    if is_far_enough {
        let target_vector = Vector2::new(vector.x(), vector.y());
        let direction = target_vector.normalize();
        rb.set_linvel(direction * movement.speed, true);
    } else {
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
    }
}

pub fn get_new_position(x: f32, y: f32, radius: f32) -> Vec3 {
    let random_x = thread_rng().gen_range(x - radius, x + radius);
    let random_y = thread_rng().gen_range(y - radius, y + radius);
    Vec3::new(random_x, random_y, 0.0)
}

pub fn run_movement_tick(movement: &mut VillagerMovement, delta: f32) -> f32 {
    if movement.tick > 0.0 {
        return movement.tick - delta;
    } else {
        return movement.tick;
    }
}
