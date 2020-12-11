use super::{
    storage::{StorageRead, StorageWithdraw},
    storage_data::{StorageData, StorageDataRead},
};
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::{enums::get_resources_list, enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
};
use bevy::ecs::{Mut, Query, ResMut};
use bevy_rapier2d::rapier::geometry::Proximity;
use enum_map::EnumMap;

#[derive(Copy, Clone)]
pub struct Stockpile {
    pub storage_data: StorageData,
    pub updated_on_startup: bool,
}

impl Stockpile {
    pub fn new(max_capacity: i32, desired_storage: EnumMap<GameResources, i32>) -> Self {
        let building = Stockpile {
            storage_data: StorageData::new(
                max_capacity,
                desired_storage,
                get_resources_list(),
                get_resources_list(),
            ),
            updated_on_startup: false,
        };
        return building;
    }

    pub fn on_proximity_event(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        event: Proximity,
        hauler: &mut Mut<Hauler>,
    ) {
        match event {
            Proximity::Intersecting => self.on_intersect(global_storage, hauler),
            _ => (),
        }
    }

    fn on_intersect(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        hauler: &mut Mut<Hauler>,
    ) {
        match hauler.state {
            HaulerStates::Loading => {
                let removal_result = self.remove_from_storage(
                    global_storage,
                    hauler.current_resource.unwrap(),
                    hauler.amount_requested,
                );
                if let Some(remainder) = removal_result {
                    if remainder == 0 {
                        hauler.capacity = HAULER_CAPACITY;
                    } else {
                        hauler.capacity = HAULER_CAPACITY - remainder;
                    }
                }
            }
            _ => (),
        }
    }
}

impl StorageRead for Stockpile {
    fn get_storage_data_mut(&mut self) -> &mut StorageData {
        return &mut self.storage_data;
    }
    fn get_storage_data(&self) -> &StorageData {
        return &self.storage_data;
    }
}

pub fn sys_update_stockpile_storage(
    mut global_storage: ResMut<GlobalStorage>,
    mut query: Query<&mut Stockpile>,
) {
    for building in query.iter_mut() {
        if !building.updated_on_startup {
            for (resource, amount) in building.storage_data.storage.iter() {
                global_storage.update_global_storage(resource.clone(), amount.clone());
            }
        }
    }
}

impl StorageWithdraw for Stockpile {
    fn remove_from_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32> {
        let storage_data = self.get_storage_data_mut();
        if storage_data.get_storage_usage() > 0 {
            let storage_has_resources: bool = storage_data.get_stored_amount(resource) >= amount;
            if storage_has_resources {
                storage_data.storage[resource] -= amount;
                storage_data.reserved_storage[resource] -= amount;
                global_storage.update_global_storage(resource, -amount);
                return Some(0);
            } else {
                let remaining_resources = storage_data.storage[resource].clone();
                storage_data.storage[resource] = 0;
                global_storage.update_global_storage(resource, -remaining_resources);
                let amount_not_removed = amount - remaining_resources;
                return Some(amount_not_removed);
            }
        } else {
            return None;
        }
    }
}

// #[derive(Debug, Eq, PartialEq)]
// enum PossibleEntities {
//     Hauler,
//     Storage,
//     None,
// }

// pub fn sys_storage_sensors(
//     events: Res<EventQueue>,
//     mut global_storage: ResMut<GlobalStorage>,
//     mut collider_set: ResMut<ColliderSet>,
//     mut storage_query: Query<&mut Stockpile>,
//     mut hauler_query: Query<&mut Hauler>,
// ) {
//     println!("Storage: Detecting!");
//     while let Ok(proximity_event) = events.proximity_events.pop() {
//         println!("Storage: Event!");
//         let (entity1, entity2) =
//             get_entities_from_proximity_event(proximity_event, &mut collider_set);
//         let mut storage: Option<Mut<Stockpile>> = None;
//         let mut hauler: Option<Mut<Hauler>> = None;
//         let mut entity1_type: PossibleEntities = PossibleEntities::None;
//         let mut entity2_type: PossibleEntities = PossibleEntities::None;
//         if let Ok(_result) = storage_query.get_mut(Entity::from_bits(entity1)) {
//             entity1_type = PossibleEntities::Storage;
//         } else if let Ok(_result) = hauler_query.get_mut(Entity::from_bits(entity1)) {
//             entity1_type = PossibleEntities::Hauler;
//         }
//         if let Ok(_result) = storage_query.get_mut(Entity::from_bits(entity2)) {
//             entity2_type = PossibleEntities::Storage;
//         } else if let Ok(_result) = hauler_query.get_mut(Entity::from_bits(entity2)) {
//             entity2_type = PossibleEntities::Hauler;
//         }

//         match entity1_type {
//             PossibleEntities::Hauler => {
//                 hauler = Some(hauler_query.get_mut(Entity::from_bits(entity1)).unwrap())
//             }
//             PossibleEntities::Storage => {
//                 storage = Some(storage_query.get_mut(Entity::from_bits(entity1)).unwrap())
//             }
//             PossibleEntities::None => {}
//         }
//         match entity2_type {
//             PossibleEntities::Hauler => {
//                 hauler = Some(hauler_query.get_mut(Entity::from_bits(entity2)).unwrap())
//             }
//             PossibleEntities::Storage => {
//                 storage = Some(storage_query.get_mut(Entity::from_bits(entity2)).unwrap())
//             }
//             PossibleEntities::None => {}
//         }

//         let hauler_and_storage_present = !hauler.is_none() && !storage.is_none();

//         if hauler_and_storage_present {
//             &mut storage.unwrap().on_proximity_event(
//                 &mut global_storage,
//                 proximity_event.new_status,
//                 &mut hauler.unwrap(),
//             );
//         }
//     }
// }
