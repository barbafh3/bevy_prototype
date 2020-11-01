mod entities;
mod managers;
mod systems;

use bevy::asset::AssetServerError;
use bevy::prelude::*;
use entities::Warehouse;
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
        .add_default_plugins()
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
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture: Result<Handle<Texture>, AssetServerError> =
        asset_server.load("assets/ralph_wolf.png");
    commands.spawn((Warehouse,)).with(Resource { capacity: 0 });
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture.unwrap().into()),
            ..Default::default()
        })
        .with(Health { value: 0 });
}
