pub mod states;

use super::VillagerMovement;
use crate::{
    constants::{OUTDOORS_IDLE_RADIUS, VILLAGER_SPEED},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Res, ResMut},
    math::{Vec2, Vec3},
    prelude::{AssetServer, Assets, SpriteBundle, Transform},
    sprite::{ColorMaterial, Sprite},
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use states::BuilderStates;

pub struct Builder {
    pub state: BuilderStates,
    pub construction_tick: f32,
    pub requested_construction: Option<Entity>,
    pub current_construction: Option<Entity>,
    pub is_inside_building: bool,
    pub is_idle: bool,
}

impl Builder {
    pub fn new(construction_tick: f32) -> Builder {
        Builder {
            state: BuilderStates::Idle,
            construction_tick,
            requested_construction: None,
            current_construction: None,
            is_inside_building: false,
            is_idle: true,
        }
    }
}

pub fn spawn_new_builder(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    starting_position: Vec3,
) {
    let texture = asset_server.load("spearman.png");
    let builder = commands
        .spawn(SpriteBundle {
            material: materials.add(texture.into()),
            transform: Transform::from_translation(starting_position),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 1.5),
            ..Default::default()
        })
        .with(Builder::new(1.0))
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
        .user_data(builder.to_bits() as u128);
    commands.insert(builder, (rigid_body, collider));
}
