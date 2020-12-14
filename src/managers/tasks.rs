pub mod build;
pub mod haul;

use bevy::ecs::Entity;

#[derive(Default)]
pub struct GameTask;

pub struct TaskFinished {
    task: Entity,
}
