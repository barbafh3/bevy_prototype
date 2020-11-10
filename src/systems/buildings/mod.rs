use bevy::{
    ecs::Query,
    ecs::Res,
    ecs::{Commands, Entity, ResMut},
    input::Input,
    math::Vec3,
    prelude::KeyCode,
    prelude::SpriteComponents,
    prelude::Transform,
    prelude::{AssetServer, Assets},
    sprite::ColorMaterial,
};

use crate::camera::CameraData;

pub struct CurrentBuilding {
    pub entity: Option<Entity>,
}

pub fn sys_spawn_building(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut current_building: ResMut<CurrentBuilding>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keyboard_input.just_released(KeyCode::T) && current_building.entity.is_none() {
        let texture_handle = asset_server.load("warehouse.png");
        commands.spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        });
        current_building.entity = commands.current_entity();
    }
    if keyboard_input.just_released(KeyCode::Y) && current_building.entity.is_some() {
        commands.despawn(current_building.entity.unwrap());
        current_building.entity = None;
    }
}

pub fn sys_building_follow_cursor(
    camera_data: ResMut<CameraData>,
    current_building: ResMut<CurrentBuilding>,
    mut query: Query<&mut Transform>,
) {
    if current_building.entity.is_some() {
        let entity = current_building.entity.unwrap();
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = Vec3::new(
                camera_data.position.x(),
                camera_data.position.y(),
                transform.translation.z(),
            );
        }
    }
}
