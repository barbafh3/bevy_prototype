mod camera;
mod entities;
mod managers;
mod systems;

// use bevy::asset::AssetServerError;
use bevy::prelude::*;
use bevy_tilemap::{
    chunk::WorldChunk,
    map::{TileMap, WorldMap},
    ChunkTilesPlugin,
};
use camera::{custom_cursor_system, CameraData, CustomCursorState};
use entities::{player::Player, Warehouse};
use managers::{
    events::{HealthIsFive, RequireResources},
    tasks::{run_tasks, TaskManager},
    tilemap::{build_tilemap, load_atlas, MapState, TileSpriteHandles, WorldTile},
};
// use state_machine::CustomStateMachine;
use systems::{
    health::{change_health, health_changed_dispatcher, health_changed_listener},
    player::{
        states::{run_player_state, PlayerStates},
        sys_player_input,
    },
    warehouse::{receive_resource, request_haul, Resource},
};

pub const TILE_SIZE: i32 = 16;
pub const TILEMAP_HEIGHT: i32 = 50;
pub const TILEMAP_WIDTH: i32 = 50;

pub enum Collider {
    Player,
}

fn main() {
    App::build()
        .add_resource(TaskManager::new())
        .init_resource::<TileSpriteHandles>()
        .init_resource::<MapState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ChunkTilesPlugin::<
            WorldTile,
            WorldChunk<WorldTile>,
            WorldMap<WorldTile, WorldChunk<WorldTile>>,
        >::default())
        .add_event::<HealthIsFive>()
        .add_event::<RequireResources>()
        .add_startup_system(startup.system())
        .add_system(custom_cursor_system.system())
        .add_system(load_atlas.system())
        .add_system(build_tilemap.system())
        .add_system(run_player_state.system())
        .add_system(sys_player_input.system())
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
    mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    map.set_dimensions(Vec2::new(1.0, 1.0));

    commands.spawn((Warehouse,)).with(Resource { capacity: 0 });

    let camera = Camera2dComponents::default();
    let e = commands.spawn(camera).current_entity().unwrap();
    commands.insert_resource(CustomCursorState {
        cursor: Default::default(),
        camera_e: e,
    });
    commands.insert_resource(CameraData {
        position: Vec2::new(0.0, 0.0),
    });

    commands.spawn(UiCameraComponents::default());

    let texture_handle = asset_server.load("archer.png");
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0)),
            ..Default::default()
        })
        .with(Player {
            state: PlayerStates::Idle,
            speed: 40.0,
            base_movement_tick: 3.0,
            movement_tick: 3.0,
            movement_radius: 50.0,
            movement_target: get_idle_point(),
        });
}

pub fn get_idle_point() -> Vec3 {
    Vec3::new(50.0, 50.0, 0.0)
}
