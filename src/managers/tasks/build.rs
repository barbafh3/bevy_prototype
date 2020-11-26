use super::TaskFinished;
use bevy::{
    ecs::{Entity, ResMut},
    prelude::Events,
};

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

impl Build {
    fn run_task(&mut self, delta: f32, event: &mut ResMut<Events<TaskFinished>>) {}
}
