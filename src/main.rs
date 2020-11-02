mod entities;
mod managers;
mod systems;

use bevy::asset::AssetServerError;
use bevy::prelude::*;
use entities::Warehouse;
use managers::{
    events::{HealthIsFive, RequireResources},
    tasks::{run_tasks, TaskManager},
    tilemap::TileMap,
};
use systems::{
    health::{change_health, health_changed_dispatcher, health_changed_listener, Health},
    warehouse::{receive_resource, request_haul, Resource},
};

pub const TILE_SIZE: i32 = 16;

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
    // let texture: Result<Handle<Texture>, AssetServerError> =
    //     asset_server.load("assets/ralph_wolf.png");
    commands.spawn((Warehouse,)).with(Resource { capacity: 0 });
    commands.spawn((TileMap::new(
        TILE_SIZE,
        TILE_SIZE,
        1,
        &commands,
        &asset_server,
        &mut materials,
    ),));
    // commands
    //     .spawn(Camera2dComponents::default())
    //     .spawn(SpriteComponents {
    //         material: materials.add(texture.unwrap().into()),
    //         ..Default::default()
    //     })
    //     .with(Health { value: 0 });
}
