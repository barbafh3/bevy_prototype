mod entities;
mod managers;
mod systems;

use bevy::prelude::*;
use entities::Player;
use managers::{
    events::HealthIsFive,
    tasks::{run_tasks, TaskManager},
};
use systems::health::{change_health, health_changed_dispatcher, health_changed_listener, Health};

fn main() {
    App::build()
        .add_event::<HealthIsFive>()
        .add_resource(TaskManager::new())
        .add_startup_system(startup.system())
        .add_system(run_tasks.system())
        .add_system(change_health.system())
        .add_system(health_changed_dispatcher.system())
        .add_system(health_changed_listener.system())
        .add_default_plugins()
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Player,)).with(Health { value: 0 });
}
