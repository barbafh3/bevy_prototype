use crate::{characters::builder::Builder, managers::villagers::IdleVillager};

use bevy::{
    core::Time,
    ecs::{Entity, Query, Res},
    math::Vec3,
};

pub struct BuilderRequest {
    pub amount: i32,
    pub position: Vec3,
    pub construction: Entity,
}

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
    fn run_task(&mut self, delta: f32) {}
}

pub fn sys_run_build_tasks(
    time: Res<Time>,
    mut build_query: Query<(Entity, &mut Build)>,
    mut idle_query: Query<(Entity, &IdleVillager, &mut Builder)>,
) {
    for (entity, mut build) in build_query.iter_mut() {
        build.run_task(time.delta_seconds);
    }
}
