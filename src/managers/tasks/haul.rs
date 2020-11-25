use bevy::{
    ecs::{Entity, ResMut},
    prelude::Events,
};

use crate::constants::enums::{GameResources, Tasks};

use super::{TaskAction, TaskFinished};

#[derive(Debug, PartialEq)]
pub struct Haul {
    task_index: i32,
    priority: f32,
    weight: f32,
    timer: f32,
    // has_loaded: bool,
    total_resource_amount: i32,
    resource_type: GameResources,
    resource_requester: Entity,
    resource_origin: Option<Entity>,
    required_haulers: i32,
    working_haulers: i32,
    hauler_list: Vec<Entity>,
}

impl Drop for Haul {
    fn drop(&mut self) {
        println!("Dropping  {:?}", self);
    }
}

impl TaskAction for Haul {
    fn run_task(&mut self, delta: f32, event: &mut ResMut<Events<TaskFinished>>) {
        if self.timer > 0.0 {
            self.timer -= delta;
        } else {
            event.send(TaskFinished {
                task_index: self.task_index.clone(),
                task_type: Tasks::Haul,
            });
        }
    }

    fn get_task_index(&mut self) -> i32 {
        self.task_index.clone()
    }

    fn set_task_index(&mut self, index: i32) {
        self.task_index = index;
    }
}

impl Haul {
    pub fn new(
        priority: f32,
        weight: f32,
        resource_type: GameResources,
        total_resource_amount: i32,
        resource_requester: Entity,
        resource_origin: Option<Entity>,
    ) -> Haul {
        Haul {
            task_index: 0,
            priority,
            weight,
            timer: 3.0,
            resource_type,
            total_resource_amount,
            resource_requester,
            resource_origin,
            required_haulers: 0,
            working_haulers: 0,
            hauler_list: vec![],
        }
    }
}
