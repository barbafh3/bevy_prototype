pub mod states;

use self::states::HaulerStates;
use super::VillagerMovement;
use crate::{
    constants::{enums::GameResources, enums::Jobs, OUTDOORS_IDLE_RADIUS, VILLAGER_SPEED},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Res, ResMut},
    math::{Vec2, Vec3},
    prelude::{AssetServer, Assets, SpriteComponents, Transform},
    sprite::{ColorMaterial, Sprite},
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

#[derive(Debug, PartialEq)]
pub struct Hauler {
    villager_type: Jobs,
    pub state: HaulerStates,
    pub capacity: i32,
    is_idle: bool,
    pub current_haul: Option<Entity>,
    pub amount_requested: i32,
    pub current_resource: Option<GameResources>,
    pub resource_origin: Option<Entity>,
    pub resource_destination: Option<Entity>,
}

impl Hauler {
    pub fn new() -> Hauler {
        Hauler {
            villager_type: Jobs::Hauler,
            state: HaulerStates::Idle,
            capacity: 0,
            is_idle: true,
            current_haul: None,
            amount_requested: 0,
            current_resource: None,
            resource_origin: None,
            resource_destination: None,
        }
    }
}

pub fn spawn_new_hauler(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    starting_position: Vec3,
) {
    let hauler_texture = asset_server.load("horse.png");
    let hauler = commands
        .spawn(SpriteComponents {
            material: materials.add(hauler_texture.into()),
            transform: Transform::from_translation(starting_position),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 1.5),
            ..Default::default()
        })
        .with(Hauler::new())
        .with(VillagerMovement {
            speed: VILLAGER_SPEED,
            base_tick: 3.0,
            tick: 3.0,
            radius: OUTDOORS_IDLE_RADIUS,
            target: Vec3::new(0.0, 0.0, 0.0),
        })
        .with(IdleVillager)
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(hauler.to_bits() as u128);
    commands.insert(hauler, (rigid_body, collider));
}
