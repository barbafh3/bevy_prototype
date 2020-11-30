mod buildings;
mod camera;
mod characters;
mod collision;
mod constants;
mod managers;
mod utils;

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{EventQueue, RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    render::RapierRenderPlugin,
};
use bevy_tilemap::{
    chunk::WorldChunk,
    map::{TileMap, WorldMap},
    ChunkTilesPlugin,
};
use buildings::{
    stockpile::sys_stockpile_sensors,
    stockpile::sys_update_stockpile_storage,
    stockpile::Stockpile,
    storage::sys_storage_sensors,
    storage::sys_update_storage_building,
    storage::{StorageBuilding, StorageTypes},
    sys_spawn_building,
    warehouse::{states::sys_run_warehouse_states, sys_warehouse_sensors},
    CurrentBuilding,
};
use camera::{sys_cursor_position, CameraData, CustomCursorState};
use characters::{
    hauler::{states::sys_run_hauler_state, Hauler, HaulerFinished},
    player::{states::run_player_state, sys_player_input},
};
use constants::enums::GameResources;
use managers::{
    storage::GlobalStorage,
    tasks::{
        haul::{sys_close_haul_tasks, sys_run_haul_tasks},
        TaskFinished,
    },
    tilemap::{build_tilemap, load_atlas, MapState, TileSpriteHandles, WorldTile},
    villagers::sys_new_villager_requests,
    villagers::IdleVillager,
    villagers::SpawnRequest,
};

pub struct RigidBodyRotationState {
    is_locked: bool,
}

fn startup(
    mut commands: Commands,
    // mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    // mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // rapier_config.gravity = Vector2::new(5.0, 5.0);
    // tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    // map.set_dimensions(Vec2::new(3.0, 3.0));

    let camera = Camera2dComponents::default();
    let e = commands.spawn(camera).current_entity().unwrap();
    commands.insert_resource(CustomCursorState {
        cursor: Default::default(),
        camera_e: e,
    });
    commands.insert_resource(CameraData {
        position: Vec2::new(0.0, 0.0),
    });

    commands.insert_resource(CurrentBuilding { entity: None });

    commands.spawn(UiCameraComponents::default());

    let idle_point_texture = asset_server.load("flag.png");
    let idle_point = get_idle_point();
    commands.spawn(SpriteComponents {
        material: materials.add(idle_point_texture.into()),
        transform: Transform::from_translation(Vec3::new(idle_point.x(), idle_point.y(), 100.0)),
        sprite: Sprite::new(Vec2::new(16.0, 16.0) * 2.0),
        ..Default::default()
    });

    let stockpile_texture = asset_server.load("stockpile.png");
    let mut storage: HashMap<GameResources, i32> = HashMap::new();
    storage.insert(GameResources::Wood, 100);
    storage.insert(GameResources::Stone, 0);
    storage.insert(GameResources::Plank, 0);
    storage.insert(GameResources::StoneBrick, 0);
    let stockpile = commands
        .spawn(SpriteComponents {
            material: materials.add(stockpile_texture.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 2.0),
            ..Default::default()
        })
        .with(StorageBuilding::new(StorageTypes::Stockpile, 1000, storage))
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(stockpile.to_bits() as u128);
    commands.insert(stockpile, (rigid_body, collider));

    let hauler_texture = asset_server.load("horse.png");
    let hauler = commands
        .spawn(SpriteComponents {
            material: materials.add(hauler_texture.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0) * 1.5),
            ..Default::default()
        })
        .with(Hauler::new(50.0, 3.0, 20.0))
        .with(IdleVillager)
        .current_entity()
        .unwrap();
    let rigid_body2 = RigidBodyBuilder::new_dynamic();
    let collider2 = ColliderBuilder::cuboid(5.0, 5.0)
        .sensor(true)
        .user_data(hauler.to_bits() as u128);
    commands.insert(hauler, (rigid_body2, collider2));

    // let rigid_body = RigidBodyBuilder::new_dynamic()
    //     .translation(0.0, 20.0)
    //     .can_sleep(false);
    // let collider = ColliderBuilder::ball(5.0);
    // commands.spawn((rigid_body, collider));

    // let rigid_body1 = RigidBodyBuilder::new_static().can_sleep(false);
    // let collider1 = ColliderBuilder::cuboid(1000.0, 1.0).sensor(true);
    // commands.spawn((rigid_body1, collider1));
}

fn load_plugins(app: &mut AppBuilder) {
    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin);
    // .add_plugin(ChunkTilesPlugin::<
    //     WorldTile,
    //     WorldChunk<WorldTile>,
    //     WorldMap<WorldTile, WorldChunk<WorldTile>>,
    // >::default());
}

fn load_resources(app: &mut AppBuilder) {
    app.add_resource(GlobalStorage::new())
        .add_resource(RigidBodyRotationState { is_locked: false });
    // .init_resource::<TileSpriteHandles>()
    // .init_resource::<MapState>();
}

fn load_startup_systems(app: &mut AppBuilder) {
    app.add_startup_system(startup.system());
}

fn load_systems(app: &mut AppBuilder) {
    core_systems(app);
    // tilemap_systems(app);
    building_systems(app);
    task_systems(app);
    player_systems(app);
    villager_systems(app);
}

fn load_events(app: &mut AppBuilder) {
    app.add_event::<HaulerFinished>()
        .add_event::<SpawnRequest>()
        .add_event::<TaskFinished>();
}

fn core_systems(app: &mut AppBuilder) {
    app.add_system(sys_cursor_position.system());
}

fn task_systems(app: &mut AppBuilder) {
    app.add_system(sys_run_haul_tasks.system())
        .add_system(sys_close_haul_tasks.system());
}

fn tilemap_systems(app: &mut AppBuilder) {
    app.add_system(load_atlas.system())
        .add_system(build_tilemap.system());
}

fn building_systems(app: &mut AppBuilder) {
    app.add_system(sys_spawn_building.system());
    app.add_system(sys_run_warehouse_states.system());
    app.add_system(sys_update_stockpile_storage.system());
    app.add_system(sys_stockpile_sensors.system());
    app.add_system(sys_storage_sensors.system());
    app.add_system(sys_update_storage_building.system());
    app.add_system(sys_warehouse_sensors.system());
}

fn player_systems(app: &mut AppBuilder) {
    app.add_system(run_player_state.system())
        .add_system(sys_player_input.system());
}

fn villager_systems(app: &mut AppBuilder) {
    app.add_system(sys_run_hauler_state.system())
        .add_system(sys_new_villager_requests.system());
}

pub fn get_idle_point() -> Vec2 {
    Vec2::new(100.0, 100.0)
}

fn print_events(events: Res<EventQueue>) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}

fn main() {
    let mut app = App::build();
    load_events(&mut app);
    load_resources(&mut app);
    load_plugins(&mut app);
    load_startup_systems(&mut app);
    load_systems(&mut app);
    app.add_system(print_events.system());
    app.run();
}
