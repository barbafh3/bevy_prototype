// use crate::{
//     characters::hauler::Hauler, constants::enums::GameResources,
//     managers::events::get_entities_from_proximity_event,
// };
// use bevy::ecs::{Entity, Query, ResMut};
// use bevy_rapier2d::{physics::EventQueue, rapier::geometry::ColliderSet};
// use std::collections::HashMap;

// pub mod states;

// #[derive(Clone)]
// pub enum BuildingStates {
//     Placing,
//     Loading,
//     Construction,
//     Done,
// }

// #[derive(Clone)]
// pub struct Building {
//     pub state: BuildingStates,
//     pub required_resources: HashMap<GameResources, i32>,
//     pub construction_time: f32,
//     pub sprite_added: bool,
//     pub is_position_set: bool,
// }

// impl Building {
//     pub fn new(required_resources: HashMap<GameResources, i32>) -> Building {
//         Building {
//             state: BuildingStates::Placing,
//             required_resources,
//             construction_time: 10.0,
//             sprite_added: false,
//             is_position_set: false,
//         }
//     }

//     pub fn on_proximity_event(&mut self) {}
// }

// pub fn sys_building_sensors(
//     events: ResMut<EventQueue>,
//     mut collider_set: ResMut<ColliderSet>,
//     mut building_query: Query<&mut Building>,
//     mut hauler_query: Query<&mut Hauler>,
// ) {
//     while let Ok(proximity_event) = events.proximity_events.pop() {
//         // let mut building: Option<Building> = None;
//         // let mut hauler: Option<Hauler> = None;
//         // let (entity1, entity2) =
//         //     get_entities_from_proximity_event(proximity_event, &mut collider_set);
//         // if let Ok(building_result) = building_query.get_mut(Entity::from_bits(entity1)) {
//         //     match building {
//         //         None => building = Some(building_result.clone()),
//         //         _ => (),
//         //     }
//         // }
//         // if let Ok(building_result) = building_query.get_mut(Entity::from_bits(entity2)) {
//         //     match building {
//         //         None => building = Some(building_result.clone()),
//         //         _ => (),
//         //     }
//         // }
//         // if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity1)) {
//         //     match hauler {
//         //         None => hauler = Some(*hauler_result),
//         //         _ => (),
//         //     }
//         // }
//         // if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity2)) {
//         //     match hauler {
//         //         None => hauler = Some(*hauler_result),
//         //         _ => (),
//         //     }
//         // }

//         // if !hauler.is_none() && !building.is_none() {
//         //     building.unwrap().on_proximity_event();
//         // }
//     }
// }
