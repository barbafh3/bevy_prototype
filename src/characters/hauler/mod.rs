pub mod states;

use bevy::math::Vec3;

use self::states::HaulerStates;
use crate::constants::{enums::GameResources, tasks::HAULER_CAPACITY};

pub struct Hauler {
    pub state: HaulerStates,
    pub speed: f32,
    pub base_movement_tick: f32,
    pub movement_tick: f32,
    pub movement_radius: f32,
    pub movement_target: Vec3,
    pub capacity: i32,
    pub current_resource: Option<GameResources>,
}

impl Hauler {
    pub fn new(speed: f32, base_movement_tick: f32, movement_radius: f32) -> Hauler {
        Hauler {
            state: HaulerStates::Idle,
            speed: speed,
            base_movement_tick,
            movement_tick: 0.0,
            movement_radius,
            movement_target: Vec3::new(0.0, 0.0, 0.0),
            capacity: HAULER_CAPACITY,
            current_resource: None,
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
}
