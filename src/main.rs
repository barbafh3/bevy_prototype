mod buildings;
mod camera;
mod characters;
mod collision;
mod managers;

// use bevy::asset::AssetServerError;
use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{EventQueue, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    render::RapierRenderPlugin,
};
use bevy_tilemap::{
    chunk::WorldChunk,
    map::{TileMap, WorldMap},
    ChunkTilesPlugin,
};
use camera::{sys_cursor_position, CameraData, CustomCursorState};
use managers::{
    tasks::{run_tasks, TaskManager},
    tilemap::{build_tilemap, load_atlas, MapState, TileSpriteHandles, WorldTile},
};
// use state_machine::CustomStateMachine;
use buildings::{sys_spawn_building, warehouse::run_warehouse_states, CurrentBuilding};
use characters::{
    player::Player,
    player::{
        states::{run_player_state, PlayerStates},
        sys_player_input,
    },
};

pub const TILE_SIZE: i32 = 16;
pub const TILEMAP_HEIGHT: i32 = 50;
pub const TILEMAP_WIDTH: i32 = 50;

pub enum Collider {
    Player,
}

fn main() {
    let mut app = App::build();
    app.add_startup_system(startup.system());
    load_resources(&mut app);
    load_plugins(&mut app);
    load_systems(&mut app);
    app.run();
}

fn startup(
    mut commands: Commands,
    // mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    // mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    // map.set_dimensions(Vec2::new(1.0, 1.0));

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

    let texture_handle = asset_server.load("archer.png");
    let rigid_body2 = RigidBodyBuilder::new_kinematic()
        .mass(0.0)
        .translation(50.0, 50.0);
    let collider2 = ColliderBuilder::ball(10.0).sensor(true);
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
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
        })
        .with_bundle((rigid_body2, collider2));
    let rigid_body1 = RigidBodyBuilder::new_static();
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
    commands.spawn((rigid_body1, collider1));
}

fn load_resources(app: &mut AppBuilder) {
    app.add_resource(TaskManager::new());
    // .init_resource::<TileSpriteHandles>()
    // .init_resource::<MapState>();
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

fn load_systems(app: &mut AppBuilder) {
    core_systems(app);
    // tilemap_systems(app);
    warehouse_systems(app);
    player_systems(app);
    app.add_system(run_tasks.system());
}

fn core_systems(app: &mut AppBuilder) {
    app.add_system(sys_spawn_building.system())
        .add_system(print_events.system())
        .add_system(sys_cursor_position.system());
}

// fn tilemap_systems(app: &mut AppBuilder) {
//     app.add_system(load_atlas.system())
//         .add_system(build_tilemap.system());
// }

fn warehouse_systems(app: &mut AppBuilder) {
    app.add_system(run_warehouse_states.system());
}

fn player_systems(app: &mut AppBuilder) {
    app.add_system(run_player_state.system())
        .add_system(sys_player_input.system());
}

pub fn get_idle_point() -> Vec3 {
    Vec3::new(50.0, 50.0, 0.0)
}

fn print_events(events: Res<EventQueue>) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}
