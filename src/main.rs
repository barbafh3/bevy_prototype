mod entities;
mod managers;
mod systems;

use bevy::prelude::*;
use entities::{Player, Warehouse};
use managers::{
    events::{HealthIsFive, RequireResources},
    tasks::{run_tasks, TaskManager},
};
use systems::{
    health::{change_health, health_changed_dispatcher, health_changed_listener, Health},
    warehouse::{receive_resource, request_haul, Resource},
};

fn main() {
    App::build()
        .add_event::<HealthIsFive>()
        .add_event::<RequireResources>()
        .add_resource(TaskManager::new())
        .add_startup_system(startup.system())
        .add_system(receive_resource.system())
        .add_system(request_haul.system())
        .add_system(run_tasks.system())
        .add_system(change_health.system())
        .add_system(health_changed_dispatcher.system())
        .add_system(health_changed_listener.system())
        .add_default_plugins()
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Player,)).with(Health { value: 0 });
    commands.spawn((Warehouse,)).with(Resource { capacity: 0 });
}
