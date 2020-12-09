pub mod states;

use crate::get_idle_point;

use super::{get_new_position, normalize, IdleMovement, VillagerMovement};
use bevy::{
    ecs::{Entity, Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
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
            movement_target: Vec3::new(0.0, 0.0, 0.0),
            requested_construction: None,
            current_construction: None,
            is_inside_building: false,
            is_idle: true,
        }
    }
}

impl IdleMovement for Builder {
    fn idle_move(
        &mut self,
        delta: f32,
        transform: &Transform,
        rb_set: &mut ResMut<RigidBodySet>,
        rb_handle: Mut<RigidBodyHandleComponent>,
    ) {
        let rb_index = rb_handle.handle();
        let rb = rb_set.get_mut(rb_index).unwrap();
        self.movement.tick = run_builder_movement_tick(self, delta);
        let can_change_target = self.movement.tick <= 0.0;
        if can_change_target {
            self.movement_target = get_new_position(
                get_idle_point().x(),
                get_idle_point().y(),
                self.movement.radius.clone(),
            );
            self.movement.tick = self.movement.base_tick.clone();
        }

        let target_vector = self.movement_target - transform.translation;
        let is_far_enough = target_vector.x().abs() > 2.0 && target_vector.y().abs() > 2.0;
        if is_far_enough {
            let direction = normalize(target_vector);
            rb.set_linvel(direction * self.movement.speed, true);
        } else {
            rb.set_linvel(Vector2::new(0.0, 0.0), true);
        }
    }
}

pub fn run_builder_movement_tick(builder: &mut Builder, delta: f32) -> f32 {
    if builder.movement.tick > 0.0 {
        return builder.movement.tick - delta;
    } else {
        return builder.movement.tick;
    }
}
