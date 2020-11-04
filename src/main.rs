mod entities;
mod managers;
mod systems;

// use bevy::asset::AssetServerError;
use bevy::prelude::*;
use entities::Warehouse;
use managers::{
    events::{HealthIsFive, RequireResources},
    tasks::{run_tasks, TaskManager},
    tilemap::TileMap,
};
use systems::{
    health::{change_health, health_changed_dispatcher, health_changed_listener},
    warehouse::{receive_resource, request_haul, Resource},
};

pub const TILE_SIZE: i32 = 16;
pub const TILEMAP_HEIGHT: i32 = 50;
pub const TILEMAP_WIDTH: i32 = 50;

// struct GameState {
//     tilemap_loaded: bool,
// }

// impl GameState {
//     pub fn new() -> GameState {
//         GameState {
//             tilemap_loaded: false,
//         }
//     }
// }

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<HealthIsFive>()
        .add_event::<RequireResources>()
        .add_resource(TaskManager::new())
        // .add_resource(GameState::new())
        .add_resource(TileMap::new(0, 1))
        .add_startup_system(startup.system())
        // .add_startup_system(generate_tiles.system())
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
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((Warehouse,)).with(Resource { capacity: 0 });
    commands.spawn(Camera2dComponents::default());
    let texture_handle = asset_server.load("tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 13);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    println!("{:?}", texture_atlas);
    // let a = texture_atlas.texture_handles.unwrap();
    // for (_texture, _index) in a.iter() {
    //     println!("Texture: {:?} - index: {}", _texture, _index);
    // }
    // commands.spawn(SpriteComponents {
    //     material: materials.add(),
    //     ..Default::default()
    // });
}

// fn generate_tiles(
//     mut commands: Commands,
//     tilemap: Res<TileMap>,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let texture = asset_server.load("ralph_wolf.png");
//     let half_width = TILEMAP_WIDTH / 2;
//     let half_height = TILEMAP_HEIGHT / 2;
//     for x in -half_width..half_width {
//         for y in -half_height..half_height {
//             commands
//                 .spawn(SpriteComponents {
//                     material: materials.add(texture.clone().into()),
//                     transform: Transform {
//                         translation: Vec3::new((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.0),
//                         scale: Vec3::new(0.06, 0.06, 0.0),
//                         rotation: Quat::from_rotation_x(0.0),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 })
//                 .with(Tile {
//                     tilemap_id: tilemap.id,
//                     position: Vec2::new(x as f32, x as f32),
//                 });
//         }
//     }
// }
