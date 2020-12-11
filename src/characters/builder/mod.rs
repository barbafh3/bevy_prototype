pub mod states;

use crate::{constants::OUTDOORS_IDLE_RADIUS, get_idle_point, managers::villagers::IdleVillager};

use super::{get_new_position, VillagerMovement};
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
use states::BuilderStates;

pub struct Builder {
    pub state: BuilderStates,
    pub construction_tick: f32,
    pub movement: VillagerMovement,
    pub movement_target: Vec3,
    pub requested_construction: Option<Entity>,
    pub current_construction: Option<Entity>,
    pub is_inside_building: bool,
    pub is_idle: bool,
}

impl Builder {
    pub fn new(
        construction_tick: f32,
        speed: f32,
        base_movement_tick: f32,
        movement_radius: f32,
    ) -> Builder {
        Builder {
            state: BuilderStates::Idle,
            construction_tick,
            movement: VillagerMovement {
                base_tick: base_movement_tick,
                tick: base_movement_tick,
                speed,
                radius: movement_radius,
            },
            movement_target: get_idle_point(),
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
        .spawn(SpriteComponents {
            material: materials.add(texture.into()),
            transform: Transform::from_translation(starting_position),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 1.5),
            ..Default::default()
        })
        .with(Builder::new(1.0, 50.0, 3.0, OUTDOORS_IDLE_RADIUS))
        .with(IdleVillager)
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(builder.to_bits() as u128);
    commands.insert(builder, (rigid_body, collider));
}

pub fn builder_idle_move(
    builder: &mut Builder,
    delta: f32,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();
    builder.movement.tick = run_builder_movement_tick(builder, delta);
    let can_change_target = builder.movement.tick <= 0.0;
    if can_change_target {
        builder.movement_target = get_new_position(
            get_idle_point().x(),
            get_idle_point().y(),
            builder.movement.radius.clone(),
        );
        builder.movement.tick = builder.movement.base_tick.clone();
    }
    let vector = builder.movement_target - transform.translation;
    let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
    if is_far_enough {
        let target_vector = Vector2::new(vector.x(), vector.y());
        let direction = target_vector.normalize();
        rb.set_linvel(direction * builder.movement.speed, true);
    } else {
        rb.set_linvel(Vector2::new(0.0, 0.0), true);
    }
}

pub fn run_builder_movement_tick(builder: &mut Builder, delta: f32) -> f32 {
    if builder.movement.tick > 0.0 {
        return builder.movement.tick - delta;
    } else {
        return builder.movement.tick;
    }
}
