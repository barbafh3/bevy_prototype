use crate::{characters::hauler::Hauler, get_idle_point};
use bevy::{
    ecs::{Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use rand::prelude::*;

pub fn state_hauler_idle(
    delta: f32,
    mut hauler: Mut<Hauler>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let mut rb = rb_set.get_mut(rb_index).unwrap();
    hauler.movement_tick = run_movement_tick(&hauler, delta);
    let can_change_target = hauler.movement_tick <= 0.0;
    if can_change_target {
        hauler.movement_target = get_new_position(
            get_idle_point().x(),
            get_idle_point().y(),
            hauler.movement_radius.clone(),
        );
        hauler.movement_tick = hauler.base_movement_tick.clone();
    }
    let target_vector = hauler.movement_target - transform.translation;
    let is_far_enough = target_vector.x().abs() > 2.0 && target_vector.y().abs() > 2.0;
    if is_far_enough {
        let direction = normalize(target_vector);
        rb.linvel = direction * hauler.speed;
    } else {
        rb.linvel = Vector2::new(0.0, 0.0);
    }
}

fn normalize(position: Vec3) -> Vector2<f32> {
    let mut direction_x: f32 = 0.0;
    let mut direction_y: f32 = 0.0;
    if position.x() > 0.0 {
        direction_x = 1.0;
    }
    if position.x() < 0.0 {
        direction_x = -1.0;
    }
    if position.y() > 0.0 {
        direction_y = 1.0;
    }
    if position.y() < 0.0 {
        direction_y = -1.0;
    }
    return Vector2::new(direction_x, direction_y);
}

fn get_new_position(x: f32, y: f32, radius: f32) -> Vec3 {
    let random_x = thread_rng().gen_range(x - radius, x + radius);
    let random_y = thread_rng().gen_range(y - radius, y + radius);
    Vec3::new(random_x, random_y, 0.0)
}

fn run_movement_tick(hauler: &Mut<Hauler>, delta: f32) -> f32 {
    if hauler.movement_tick > 0.0 {
        return hauler.movement_tick - delta;
    } else {
        return hauler.movement_tick;
    }
}
