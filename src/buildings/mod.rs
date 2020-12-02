pub mod building;
pub mod collision_detection;
pub mod stockpile;
pub mod storage;
pub mod storage_data;
pub mod warehouse;

use self::warehouse::Warehouse;
use crate::constants::enums::GameResources;
use bevy::{
    ecs::Res,
    ecs::{Commands, Entity, ResMut},
    input::Input,
    math::Vec2,
    math::Vec3,
    prelude::KeyCode,
    prelude::SpriteComponents,
    prelude::Transform,
    prelude::{AssetServer, Assets},
    sprite::ColorMaterial,
    sprite::Sprite,
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use enum_map::enum_map;

pub struct CurrentBuilding {
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct Building;

pub fn sys_spawn_building(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
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
        .spawn(SpriteComponents {
            material: materials.add(warehouse_texture.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 2.0),
            ..Default::default()
        })
        .with(Warehouse::new(1000, required_resources))
        .current_entity()
        .unwrap();
    let rigid_body2 = RigidBodyBuilder::new_dynamic().can_sleep(false);
    let collider2 = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(warehouse.to_bits() as u128);
    commands.insert(warehouse, (rigid_body2, collider2));

    current_building.entity = commands.current_entity();
}
