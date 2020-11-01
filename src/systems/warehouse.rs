use super::super::{
    entities::Warehouse,
    managers::tasks::{Haul, TaskManager},
};
use bevy::prelude::{Query, ResMut};

pub struct Resource {
    pub capacity: i32,
}

pub fn receive_resource(mut query: Query<&mut Resource>) {
    for mut resource in &mut query.iter() {
        resource.capacity += 1;
    }
}

pub fn request_haul(
    mut task_manager: ResMut<TaskManager>,
    mut query: Query<(&Warehouse, &mut Resource)>,
) {
    for (_warehouse, resource) in &mut query.iter() {
        if resource.capacity > 400 {
            let haul = Haul;
            task_manager.register_task(haul);
        }
    }
}
