pub mod states;

use self::states::HaulerStates;
use super::{get_new_position, VillagerMovement};
use crate::{
    constants::{enums::GameResources, enums::Jobs, OUTDOORS_IDLE_RADIUS},
    get_idle_point,
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, Res, ResMut},
    math::{Vec2, Vec3},
    prelude::{AssetServer, Assets, SpriteComponents, Transform},
    sprite::{ColorMaterial, Sprite},
};
use bevy_rapier2d::{
    na::Vector2,
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

#[derive(Debug, PartialEq)]
pub struct Hauler {
    villager_type: Jobs,
    pub state: HaulerStates,
    pub movement: VillagerMovement,
    pub movement_target: Vec3,
    pub capacity: i32,
    is_idle: bool,
    pub current_haul: Option<Entity>,
    pub amount_requested: i32,
    pub current_resource: Option<GameResources>,
    pub resource_origin: Option<Entity>,
    pub resource_destination: Option<Entity>,
}

impl Hauler {
    pub fn new(speed: f32, base_movement_tick: f32, movement_radius: f32) -> Hauler {
        Hauler {
            villager_type: Jobs::Hauler,
            state: HaulerStates::Idle,
            movement: VillagerMovement {
                speed: speed,
                base_tick: base_movement_tick,
                tick: 0.0,
                radius: movement_radius,
            },
            movement_target: Vec3::new(0.0, 0.0, 0.0),
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
        .with(Hauler::new(50.0, 3.0, OUTDOORS_IDLE_RADIUS))
        .with(IdleVillager)
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(hauler.to_bits() as u128);
    commands.insert(hauler, (rigid_body, collider));
}

pub fn hauler_idle_move(
    hauler: &mut Hauler,
    delta: f32,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();
    hauler.movement.tick = run_hauler_movement_tick(hauler, delta);
    let can_change_target = hauler.movement.tick <= 0.0;
    if can_change_target {
        hauler.movement_target = get_new_position(
            get_idle_point().x(),
            get_idle_point().y(),
            hauler.movement.radius.clone(),
        );
        hauler.movement.tick = hauler.movement.base_tick.clone();
    }
    let vector = hauler.movement_target - transform.translation;
    let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
    if is_far_enough {
        let target_vector = Vector2::new(vector.x(), vector.y());
        let direction = target_vector.normalize();
        rb.set_linvel(direction * hauler.movement.speed, true);
    } else {
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
    }
}

pub fn run_hauler_movement_tick(hauler: &mut Hauler, delta: f32) -> f32 {
    if hauler.movement.tick > 0.0 {
        return hauler.movement.tick - delta;
    } else {
        return hauler.movement.tick;
    }
}
