mod buildings;
mod camera;
mod characters;
mod collision;
mod managers;

use std::fmt::{self, Debug, Display};
// use bevy::asset::AssetServerError;
use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{EventQueue, RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
    rapier::{
        data::arena::Index,
        dynamics::RigidBodyBuilder,
        geometry::Proximity,
        geometry::{
            ColliderBuilder, ColliderHandle, ColliderSet, InteractionGroups, ProximityEvent,
        },
    },
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

pub struct RigidBodyRotationState {
    is_locked: bool,
}

fn main() {
    let mut app = App::build();
    load_resources(&mut app);
    load_plugins(&mut app);
    load_startup_systems(&mut app);
    load_systems(&mut app);
    app.run();
}

fn startup(
    mut commands: Commands,
    // mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    // mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let mut collision_groups = InteractionGroups::new(1, 1);
    rapier_config.gravity = Vector2::new(0.0, -10.0);
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
    let player = commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 100.0)),
            sprite: Sprite::new(Vec2::new(16.0, 16.0)),
            ..Default::default()
        })
        .with(Player {
            state: PlayerStates::Run,
            speed: 100.0,
            base_movement_tick: 3.0,
            movement_tick: 3.0,
            movement_radius: 50.0,
            movement_target: get_idle_point(),
        })
        .current_entity()
        .unwrap();
    let rigid_body = RigidBodyBuilder::new_dynamic().translation(0.0, 100.0);
    let collider = ColliderBuilder::ball(10.0).user_data(player.to_bits() as u128);
    commands.insert(player, (rigid_body, collider));

    let rigid_body1 = RigidBodyBuilder::new_static();
    let collider1 = ColliderBuilder::cuboid(1000.0, 1.0);
    commands.spawn((rigid_body1, collider1));

    // let rigid_body2 = RigidBodyBuilder::new_static().translation(0.0, 20.0);
    // let collider2 = ColliderBuilder::ball(50.0).sensor(true);
    // commands.spawn((rigid_body2, collider2));
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
        .add_plugin(RapierRenderPlugin);
    // .add_plugin(ChunkTilesPlugin::<
    //     WorldTile,
    //     WorldChunk<WorldTile>,
    //     WorldMap<WorldTile, WorldChunk<WorldTile>>,
    // >::default());
}

fn load_resources(app: &mut AppBuilder) {
    app.add_resource(TaskManager::new())
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
    warehouse_systems(app);
    player_systems(app);
    app.add_system(run_tasks.system());
}

fn core_systems(app: &mut AppBuilder) {
    app.add_system(sys_spawn_building.system())
        .add_system(lock_rigidbody_rotation.system())
        // .add_system(print_events.system())
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

fn print_events(
    events: ResMut<EventQueue>,
    mut collider_set: ResMut<ColliderSet>,
    mut query: Query<&mut Player>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);
        if let Ok(player) = query.get_mut(Entity::from_bits(entity1)) {
            println!("{}", player.on_proximity_event(proximity_event.new_status));
        }
        if let Ok(player) = query.get_mut(Entity::from_bits(entity2)) {
            println!("{}", player.on_proximity_event(proximity_event.new_status));
        }
        // println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}

pub fn get_entities_from_proximity_event(
    proximity_event: ProximityEvent,
    collider_set: &mut ResMut<ColliderSet>,
) -> (u64, u64) {
    return (
        collider_set
            .get_mut(proximity_event.collider1)
            .unwrap()
            .user_data as u64,
        collider_set
            .get_mut(proximity_event.collider2)
            .unwrap()
            .user_data as u64,
    );
}
