pub mod builder;
pub mod hauler;
pub mod villager;

use self::hauler::Hauler;
use bevy::{
    ecs::{Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use rand::*;

#[derive(Debug, PartialEq)]
pub struct VillagerMovement {
    pub speed: f32,
    pub base_tick: f32,
    pub tick: f32,
    pub radius: f32,
}

pub trait IdleMovement {
    fn idle_move(
        &mut self,
        delta: f32,
        transform: &Transform,
        rb_set: &mut ResMut<RigidBodySet>,
        rb_handle: Mut<RigidBodyHandleComponent>,
    );
}

pub fn normalize(position: Vec3) -> Vector2<f32> {
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

pub fn get_new_position(x: f32, y: f32, radius: f32) -> Vec3 {
    let random_x = thread_rng().gen_range(x - radius, x + radius);
    let random_y = thread_rng().gen_range(y - radius, y + radius);
    Vec3::new(random_x, random_y, 0.0)
}

pub fn run_movement_tick(hauler: &mut Hauler, delta: f32) -> f32 {
    if hauler.movement.tick > 0.0 {
        return hauler.movement.tick - delta;
    } else {
        return hauler.movement.tick;
    }
}
