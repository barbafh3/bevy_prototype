use bevy::math::Vec3;

pub mod enums;
pub mod tasks;
pub mod tilemap;

pub fn default_idle_point() -> Vec3 {
    Vec3::new(50.0, 50.0, 0.0)
}
