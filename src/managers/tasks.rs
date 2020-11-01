use bevy::prelude::ResMut;
// use std::sync::{Arc, Mutex};

pub trait TaskAction {
    fn run_task(&self);
}

pub struct Haul;

impl TaskAction for Haul {
    fn run_task(&self) {
        println!("Haul running");
    }
}

pub struct Build;

impl TaskAction for Build {
    fn run_task(&self) {
        println!("Build running");
    }
}

pub struct TaskManager {
    tasks: Vec<Box<dyn TaskAction + Send + Sync>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: vec![] }
    }

    pub fn register_task<T: TaskAction + 'static + Send + Sync>(&mut self, task: T) {
        self.tasks.push(Box::new(task));
    }
}

pub fn run_tasks(task_manager: ResMut<TaskManager>) {
    if task_manager.tasks.len() > 0 {
        for task in task_manager.tasks.iter() {
            task.run_task()
        }
    }
}
