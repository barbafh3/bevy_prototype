use bevy::{
    ecs::{Entity, ResMut},
    prelude::Events,
};

use crate::constants::enums::TaskTypes;

use super::{TaskAction, TaskFinished};

#[derive(Debug)]
pub struct Build {
    task_index: i32,
    priority: f32,
    weight: f32,
    has_loaded: bool,
    requested_construction: Entity,
    working_builders: i32,
    builder_list: Vec<Entity>,
}

impl Drop for Build {
    fn drop(&mut self) {
        println!("Dropping  {:?}", self);
    }
}

impl TaskAction for Build {
    fn run_task(&mut self, delta: f32, event: &mut ResMut<Events<TaskFinished>>) {}

    fn get_task_index(&mut self) -> i32 {
        self.task_index.clone()
    }

    fn set_task_index(&mut self, index: i32) {
        self.task_index = index;
    }
}

impl Build {
    fn haul_finished(&'static self, mut event: ResMut<Events<TaskFinished>>) {
        event.send(TaskFinished {
            task_index: self.task_index,
            task_type: TaskTypes::Build,
        });
    }
}
