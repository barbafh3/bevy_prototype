pub mod build;
pub mod haul;

use crate::constants::enums::Tasks;
use bevy::{
    core::Time,
    ecs::{Local, Res},
    prelude::{EventReader, Events, ResMut},
};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub trait TaskAction {
    fn run_task(&mut self, delta: f32, event: &mut ResMut<Events<TaskFinished>>);
    fn get_task_index(&mut self) -> i32;
    fn set_task_index(&mut self, index: i32);
}

pub struct TaskFinished {
    task_index: i32,
    task_type: Tasks,
}

pub struct TaskManager {
    tasks: HashMap<i32, Box<dyn TaskAction + Send + Sync>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
        }
    }

    pub fn register_task<T: TaskAction + 'static + Send + Sync>(&mut self, mut task: T) {
        self.register_task_recursive(task);
    }

    fn register_task_recursive<T: TaskAction + 'static + Send + Sync>(&mut self, mut task: T) {
        let key = rand::random::<i32>().abs();
        if self.tasks.contains_key(&key) {
            self.register_task_recursive(task);
        } else {
            task.set_task_index(key);
            self.tasks.insert(task.get_task_index(), Box::new(task));
        }
    }
}

pub fn sys_run_tasks(time: Res<Time>, mut event: ResMut<Events<TaskFinished>>) {
    let task_manager = &mut TASK_MANAGER.lock().unwrap();
    if task_manager.tasks.len() > 0 {
        for (_, task) in task_manager.tasks.iter_mut() {
            task.run_task(time.delta_seconds, &mut event);
        }
    }
}

pub fn sys_task_finished(
    mut event_reader: Local<EventReader<TaskFinished>>,
    task_finished_events: Res<Events<TaskFinished>>,
) {
    let task_manager = &mut TASK_MANAGER.lock().unwrap();
    for task_finished in event_reader.iter(&task_finished_events) {
        println!("Task list lenght: {}", task_manager.tasks.len());
        let mut task = task_manager
            .tasks
            .remove(&task_finished.task_index)
            .unwrap();
        println!("Task index: {}", task.get_task_index());
        drop(task.as_mut());
        println!(
            "Task list lenght after removal: {}",
            task_manager.tasks.len()
        );
    }
}
