pub mod warehouse;

use bevy::{
    ecs::Query,
    ecs::Res,
    ecs::{Commands, Entity, ResMut},
    input::Input,
    math::Vec3,
    prelude::KeyCode,
    prelude::MouseButton,
    prelude::SpriteComponents,
    prelude::Transform,
    prelude::{AssetServer, Assets},
    sprite::ColorMaterial,
};

use crate::camera::CameraData;

use self::warehouse::WarehouseStates;

pub struct Warehouse {
    pub state: WarehouseStates,
}

pub struct CurrentBuilding {
    pub entity: Option<Entity>,
}

pub fn sys_spawn_building(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    camera_data: Res<CameraData>,
    mut current_building: ResMut<CurrentBuilding>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut Warehouse>,
) {
    let can_spawn_building =
        keyboard_input.just_released(KeyCode::T) && current_building.entity.is_none();
    if can_spawn_building {
        let texture_handle = asset_server.load("warehouse.png");
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
            .with(Warehouse {
                state: WarehouseStates::Placing,
            });
        current_building.entity = commands.current_entity();
    }
    if mouse_input.just_released(MouseButton::Right) && current_building.entity.is_some() {
        commands.despawn(current_building.entity.unwrap());
        current_building.entity = None;
    }
    if mouse_input.just_released(MouseButton::Left) {
        if let Ok(mut warehouse) = query.get_mut(current_building.entity.unwrap()) {
            warehouse.state = WarehouseStates::Placed;
            current_building.entity = None;
        }
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
