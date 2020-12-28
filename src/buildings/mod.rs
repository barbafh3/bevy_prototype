pub mod collision_detection;
pub mod construction;
pub mod stockpile;
pub mod storage_building;
pub mod storage_data;
pub mod warehouse;
pub mod woodcuttershut;

use self::construction::{Construction, ConstructionTypes};
use crate::{camera::CameraData, constants::enums::GameResources};
use bevy::{
    ecs::Res,
    ecs::{Commands, Entity, ResMut},
    input::Input,
    math::Vec2,
    math::Vec3,
    prelude::KeyCode,
    prelude::Transform,
    prelude::{AssetServer, Assets, SpriteBundle},
    sprite::ColorMaterial,
    sprite::Sprite,
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use enum_map::enum_map;

pub struct CurrentBuilding {
    pub entity: Option<Entity>,
}

pub struct DisabledBuilding;

#[derive(Default)]
pub struct Building;

pub fn sys_spawn_building(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    camera_data: Res<CameraData>,
    asset_server: Res<AssetServer>,
    mut current_building: ResMut<CurrentBuilding>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let can_spawn_building =
        keyboard_input.just_released(KeyCode::T) && current_building.entity.is_none();
    if !can_spawn_building {
        return;
    }
    println!("Spawning building");
    let required_resources = enum_map! { GameResources::Wood => 50, _ => 0 };
    let warehouse_texture = asset_server.load("under_construction.png");
    let warehouse = commands
        .spawn(SpriteBundle {
            material: materials.add(warehouse_texture.into()),
            transform: Transform::from_translation(Vec3::new(
                camera_data.position.x,
                camera_data.position.y,
                100.0,
            )),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 2.0),
            ..Default::default()
        })
        .with(Construction::new(
            ConstructionTypes::WarehouseParams(10000),
            required_resources,
            10.0,
        ))
        .current_entity()
        .unwrap();
    let rigid_body2 = RigidBodyBuilder::new_dynamic().can_sleep(false);
    let collider2 = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(warehouse.to_bits() as u128);
    commands.insert(warehouse, (rigid_body2, collider2));
    current_building.entity = commands.current_entity();
}
