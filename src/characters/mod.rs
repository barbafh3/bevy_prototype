pub mod builder;
pub mod hauler;
pub mod villager;

use bevy::math::Vec3;
use rand::*;

#[derive(Debug, PartialEq)]
pub struct VillagerMovement {
    pub speed: f32,
    pub base_tick: f32,
    pub tick: f32,
    pub radius: f32,
}

pub fn get_new_position(x: f32, y: f32, radius: f32) -> Vec3 {
    let random_x = thread_rng().gen_range(x - radius, x + radius);
    let random_y = thread_rng().gen_range(y - radius, y + radius);
    Vec3::new(random_x, random_y, 0.0)
}
