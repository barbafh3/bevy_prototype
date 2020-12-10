pub mod states;

use self::states::HaulerStates;
use super::{get_new_position, IdleMovement, VillagerMovement};
use crate::{
    constants::{enums::GameResources, enums::Jobs},
    get_idle_point,
};
use bevy::{
    ecs::{Entity, Mut, ResMut},
    math::Vec3,
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

// pub struct HaulerFinished {
//     task: Entity,
//     hauler: Entity,
//     amount_delivered: i32,
// }

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

    pub fn deliver_resource(&mut self) -> Option<(GameResources, i32)> {
        match self.current_resource {
            Some(resource) => {
                let result = (resource, self.capacity);
                self.capacity = 0;
                self.current_resource = None;
                return Some(result);
            }
            None => None,
        }
    }

    pub fn take_resources(&mut self, amount: i32) {
        self.capacity = amount;
    }
}

impl IdleMovement for Hauler {
    fn idle_move(
        &mut self,
        delta: f32,
        transform: &Transform,
        rb_set: &mut ResMut<RigidBodySet>,
        rb_handle: Mut<RigidBodyHandleComponent>,
    ) {
        let rb_index = rb_handle.handle();
        let rb = rb_set.get_mut(rb_index).unwrap();
        self.movement.tick = run_hauler_movement_tick(self, delta);
        let can_change_target = self.movement.tick <= 0.0;
        if can_change_target {
            self.movement_target = get_new_position(
                get_idle_point().x(),
                get_idle_point().y(),
                self.movement.radius.clone(),
            );
            self.movement.tick = self.movement.base_tick.clone();
        }

        let vector = self.movement_target - transform.translation;
        let is_far_enough = vector.x().abs() > 2.0 && vector.y().abs() > 2.0;
        if is_far_enough {
            let target_vector = Vector2::new(vector.x(), vector.y());
            let direction = target_vector.normalize();
            rb.set_linvel(direction * self.movement.speed, true);
        } else {
            rb.set_linvel(Vector2::new(0.0, 0.0), true);
        }
    }
}

pub fn run_hauler_movement_tick(hauler: &mut Hauler, delta: f32) -> f32 {
    if hauler.movement.tick > 0.0 {
        return hauler.movement.tick - delta;
    } else {
        return hauler.movement.tick;
    }
}
