mod buildings;
mod camera;
mod characters;
mod collision;
mod constants;
mod managers;
mod utils;

use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    render::RapierRenderPlugin,
};
use bevy_tilemap::{
    chunk::WorldChunk,
    map::{TileMap, WorldMap},
    ChunkTilesPlugin,
};
use buildings::{sys_spawn_building, warehouse::states::sys_run_warehouse_states, CurrentBuilding};
use camera::{sys_cursor_position, CameraData, CustomCursorState};
use characters::{
    hauler::{states::sys_run_hauler_state, Hauler},
    player::{states::run_player_state, sys_player_input},
};
use managers::{
    tasks::{sys_run_tasks, sys_task_finished, TaskFinished, TaskManager},
    tilemap::{build_tilemap, load_atlas, MapState, TileSpriteHandles, WorldTile},
};
use utils::collision_events::sys_print_events;

pub struct RigidBodyRotationState {
    is_locked: bool,
}

fn startup(
    mut commands: Commands,
    mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector2::new(0.0, 0.0);
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    map.set_dimensions(Vec2::new(1.0, 1.0));

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

    // let player_texture = asset_server.load("archer.png");
    // let player = commands
    //     .spawn(SpriteComponents {
    //         material: materials.add(player_texture.into()),
    //         transform: Transform::from_translation(Vec3::new(0.0, 100.0, 100.0)),
    //         sprite: Sprite::new(Vec2::new(16.0, 16.0)),
    //         ..Default::default()
    //     })
    //     .with(Player {
    //         state: PlayerStates::Run,
    //         speed: 100.0,
    //         base_movement_tick: 3.0,
    //         movement_tick: 3.0,
    //         movement_radius: 50.0,
    //         movement_target: default_idle_point(),
    //     })
    //     .current_entity()
    //     .unwrap();
    // let rigid_body = RigidBodyBuilder::new_dynamic()
    //     .translation(0.0, 100.0)
    //     .can_sleep(false);
    // let collider = ColliderBuilder::ball(10.0).user_data(player.to_bits() as u128);
    // commands.insert(player, (rigid_body, collider));

    let hauler_texture = asset_server.load("horse.png");
    let hauler = commands
        .spawn(SpriteComponents {
            material: materials.add(hauler_texture.into()),
            transform: Transform::from_translation(Vec3::new(-100.0, 100.0, 100.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0)),
            ..Default::default()
        })
        .with(Hauler::new(50.0, 3.0, 20.0))
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 100.0)
        .can_sleep(false);
    let collider = ColliderBuilder::ball(10.0).user_data(hauler.to_bits() as u128);
    commands.insert(hauler, (rigid_body, collider));

    // let rigid_body1 = RigidBodyBuilder::new_static();
    // let collider1 = ColliderBuilder::cuboid(1000.0, 1.0);
    // commands.spawn((rigid_body1, collider1));
}

fn lock_rigidbody_rotation(
    mut lock_state: ResMut<RigidBodyRotationState>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<&mut RigidBodyHandleComponent>,
) {
    if !lock_state.is_locked {
        println!("Query length is {}", query.iter_mut().len());
        for rb_handle in query.iter_mut() {
            let rb_index = rb_handle.handle();
            let mut rb = rb_set.get_mut(rb_index).unwrap();
            rb.mass_properties.inv_principal_inertia_sqrt = 0.0;
        }
        lock_state.is_locked = true;
    }
}

fn load_plugins(app: &mut AppBuilder) {
    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugin(ChunkTilesPlugin::<
            WorldTile,
            WorldChunk<WorldTile>,
            WorldMap<WorldTile, WorldChunk<WorldTile>>,
        >::default());
}

fn load_resources(app: &mut AppBuilder) {
    app.add_resource(TaskManager::new())
        .add_resource(RigidBodyRotationState { is_locked: false })
        .init_resource::<TileSpriteHandles>()
        .init_resource::<MapState>();
}

fn load_startup_systems(app: &mut AppBuilder) {
    app.add_startup_system(startup.system());
}

fn load_systems(app: &mut AppBuilder) {
    core_systems(app);
    tilemap_systems(app);
    warehouse_systems(app);
    player_systems(app);
    villager_systems(app);
    app.add_system(sys_run_tasks.system());
}

fn core_systems(app: &mut AppBuilder) {
    app.add_system(sys_spawn_building.system())
        .add_system(lock_rigidbody_rotation.system())
        .add_system(sys_print_events.system())
        .add_system(sys_task_finished.system())
        .add_system(sys_cursor_position.system());
}

fn tilemap_systems(app: &mut AppBuilder) {
    app.add_system(load_atlas.system())
        .add_system(build_tilemap.system());
}

fn warehouse_systems(app: &mut AppBuilder) {
    app.add_system(sys_run_warehouse_states.system());
}

fn player_systems(app: &mut AppBuilder) {
    app.add_system(run_player_state.system())
        .add_system(sys_player_input.system());
}

fn villager_systems(app: &mut AppBuilder) {
    app.add_system(sys_run_hauler_state.system());
}

pub fn get_idle_point() -> Vec2 {
    Vec2::new(100.0, 100.0)
}

fn main() {
    let mut app = App::build();
    app.add_event::<TaskFinished>();
    load_resources(&mut app);
    load_plugins(&mut app);
    load_startup_systems(&mut app);
    load_systems(&mut app);
    app.run();
}
