pub mod warehouse;

use bevy::{
    // ecs::Query,
    ecs::Res,
    ecs::{Commands, Entity, ResMut},
    input::Input,
    math::Vec3,
    prelude::KeyCode,
    // prelude::MouseButton,
    prelude::SpriteComponents,
    prelude::Transform,
    prelude::{AssetServer, Assets},
    sprite::ColorMaterial,
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use crate::camera::CameraData;

use self::warehouse::Warehouse;

pub struct CurrentBuilding {
    pub entity: Option<Entity>,
}

pub fn sys_spawn_building(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // mouse_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    camera_data: Res<CameraData>,
    mut current_building: ResMut<CurrentBuilding>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut query: Query<&mut Warehouse>,
) {
    let can_spawn_building =
        keyboard_input.just_released(KeyCode::T) && current_building.entity.is_none();
    if can_spawn_building {
        let texture_handle = asset_server.load("under_construction.png");
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0.0, 3.0);
        let collider = ColliderBuilder::ball(0.5);
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_translation(Vec3::new(
                    camera_data.position.x(),
                    camera_data.position.y(),
                    10.0,
                )),
                ..Default::default()
            })
            .with(Warehouse::new())
            .with((rigid_body, collider));
        current_building.entity = commands.current_entity();
    }
}

// pub fn sys_building_follow_cursor(
//     camera_data: ResMut<CameraData>,
//     current_building: ResMut<CurrentBuilding>,
//     mut query: Query<&mut Transform>,
// ) {
//     if current_building.entity.is_some() {
//         let entity = current_building.entity.unwrap();
//         if let Ok(mut transform) = query.get_mut(entity) {
//             transform.translation = Vec3::new(
//                 camera_data.position.x(),
//                 camera_data.position.y(),
//                 transform.translation.z(),
//             );
//         }
//     }
// }
