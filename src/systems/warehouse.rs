use super::super::{
    entities::Warehouse,
    managers::tasks::{Haul, TaskManager},
};
use bevy::prelude::{Query, ResMut};

pub struct Resource {
    pub capacity: i32,
}

pub fn receive_resource(mut _query: Query<&Resource>) {
    // for mut resource in query.iter() {
    //     resource.capacity += 1;
    // }
}

pub fn request_haul(mut task_manager: ResMut<TaskManager>, query: Query<(&Warehouse, &Resource)>) {
    for (_warehouse, resource) in query.iter() {
        if resource.capacity > 400 {
            let haul = Haul;
            task_manager.register_task(haul);
        }
    }
}
