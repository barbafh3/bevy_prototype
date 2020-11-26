#![feature(type_name_of_val)]
use bevy::{
    ecs::Local,
    ecs::Query,
    ecs::{Commands, Entity, Res, ResMut},
    math::{Vec2, Vec3},
    prelude::EventReader,
    prelude::Events,
    prelude::{AssetServer, Assets, SpriteComponents, Transform},
    sprite::ColorMaterial,
    sprite::Sprite,
};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use crate::{characters::hauler::Hauler, constants::enums::Jobs};

use super::tasks::TaskManager;

pub struct SpawnRequest {
    job: Jobs,
    position: Vec3,
}

#[derive(Default)]
pub struct IdleVillager;

pub fn sys_new_villager_requests(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_request_reader: Local<EventReader<SpawnRequest>>,
    spawn_request_event: Res<Events<SpawnRequest>>,
) {
    for spawn_event in spawn_request_reader.iter(&spawn_request_event) {
        spawn_villager(
            &mut commands,
            &asset_server,
            &mut materials,
            spawn_event.job,
            spawn_event.position,
        );
    }
}

pub fn sys_idle_villager_listing(
    mut task_manager: ResMut<TaskManager>,
    query: Query<(Entity, &IdleVillager)>,
) {
    let idle_villager_list = query.iter().map(|(entity, _)| entity).collect();
    task_manager.push_idle_list(idle_villager_list);
}

pub fn spawn_villager(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    villager_type: Jobs,
    position: Vec3,
) -> Option<Entity> {
    match villager_type {
        Jobs::Hauler => {
            let hauler_texture = asset_server.load("horse.png");
            let hauler = commands
                .spawn(SpriteComponents {
                    material: materials.add(hauler_texture.into()),
                    transform: Transform::from_translation(position),
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
            let new_hauler_entity = commands.current_entity().unwrap();
            return Some(new_hauler_entity);
        }
        _ => None,
    }
}

// pub struct VillagerManager {
//     villagers: Vec<Entity>,
// }

// lazy_static! {
//     pub static ref VILLAGER_MANAGER: Mutex<VillagerManager> = Mutex::new(VillagerManager::new());
// }

// impl VillagerManager {
//     pub fn new() -> VillagerManager {
//         VillagerManager { villagers: vec![] }
//     }

//     pub fn register_villager(&mut self, mut villager: Entity) {
//         if !self.villagers.contains(&villager) {
//             self.villagers.push(villager);
//         }
//     }

//     pub fn request_new_villager(
//         &mut self,
//         commands: &mut Commands,
//         asset_server: &Res<AssetServer>,
//         materials: &mut ResMut<Assets<ColorMaterial>>,
//         villager_type: Jobs,
//         mut query_set: QuerySet<(Query<&mut Hauler>, Query<(&Villager, &Transform)>)>,
//     ) {
//         let mut selected_villager: Option<(&i32, &Box<dyn IndexedVillager + Send + Sync>)> = None;
//         for villager_entity in self.villagers.iter() {
//             if let (villager, transform) = query_set.q1().iter() {}
//             // let is_villager_type_equal = villager.get_villager_type() == Jobs::Villager;
//             // if is_villager_type_equal {
//             //     let query: &Query<(&Villager, &Transform)> = query_set.q1();
//             // }
//         }
//     }

// }

// pub trait IndexedVillager {
//     fn get_villager_index(&self) -> i32;
//     fn set_villager_index(&mut self, index: i32);
//     fn get_villager_type(&self) -> Jobs;
//     fn is_idle(&self) -> bool;
//     fn set_status(&mut self, idle: bool);
// }
