use bevy::{
    ecs::{Mut, Res},
    math::Vec3,
    prelude::Transform,
};

use crate::camera::CameraData;

pub fn state_placing_warehouse(camera_data: &Res<CameraData>, mut transform: Mut<Transform>) {
    println!("Warehouse is beign placed...");
    transform.translation = Vec3::new(
        camera_data.position.x(),
        camera_data.position.y(),
        transform.translation.z(),
    );
}
