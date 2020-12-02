use super::storage_data::{StorageData, StorageDataRead};
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::{enums::get_resources_list, enums::GameResources, tasks::HAULER_CAPACITY},
    managers::{events::get_entities_from_proximity_event, storage::GlobalStorage},
};
use bevy::ecs::{Entity, Mut, Query, ResMut};
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::geometry::{ColliderSet, Proximity},
};
use enum_map::EnumMap;

#[derive(Copy, Clone)]
pub struct StorageBuilding {
    pub storage_type: StorageTypes,
    pub storage_data: StorageData,
    pub updated_on_startup: bool,
}

#[derive(Copy, Clone)]
pub enum StorageTypes {
    Stockpile,
    Warehouse,
}

impl StorageBuilding {
    pub fn new(
        storage_type: StorageTypes,
        max_capacity: i32,
        desired_storage: EnumMap<GameResources, i32>,
    ) -> Self {
        let building = StorageBuilding {
            storage_type,
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
        hauler: &mut Hauler,
    ) {
        println!("StorageBuilding: EVENT!");
        match event {
            Proximity::Intersecting => self.on_intersect(global_storage, hauler),
            _ => (),
        }
    }

    fn on_intersect(&mut self, global_storage: &mut ResMut<GlobalStorage>, hauler: &mut Hauler) {
        println!("Stockpile: Hauler Intersect!");
        println!("Stockpile: Hauler is {:?}", hauler.state);
        match hauler.state {
            HaulerStates::Loading => {
                let removal_result = self.remove_from_storage(
                    global_storage,
                    hauler.current_resource.unwrap(),
                    hauler.amount_requested,
                );
                if let Some(remainder) = removal_result {
                    println!("Stockpile: Remainder = {}", remainder);
                    if remainder == 0 {
                        hauler.take_resources(HAULER_CAPACITY);
                    } else {
                        hauler.take_resources(HAULER_CAPACITY - remainder)
                    }
                }
            }
            _ => (),
        }
    }
}

impl StorageRead for StorageBuilding {
    fn get_storage_data_mut(&mut self) -> &mut StorageData {
        return &mut self.storage_data;
    }
    fn get_storage_data(&self) -> &StorageData {
        return &self.storage_data;
    }
}

pub trait StorageRead {
    fn get_storage_data(&self) -> &StorageData;
    fn get_storage_data_mut(&mut self) -> &mut StorageData;
}

pub trait StorageInsert {
    fn add_to_storage(
        &mut self,
        // storage_manager: &mut ResMut<StorageManager>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32>;
}

pub trait StorageWithdraw {
    fn remove_from_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32>;
}

pub trait ResourceReservation {
    fn reserve_resource(&mut self, resource: GameResources, amount: i32) -> bool;
    fn add_incoming_resource(&mut self, resource: GameResources, amount: i32) -> bool;
}

pub fn sys_update_storage_building(
    mut global_storage: ResMut<GlobalStorage>,
    mut query: Query<&mut StorageBuilding>,
) {
    for mut building in query.iter_mut() {
        if !building.updated_on_startup {
            match building.storage_type {
                StorageTypes::Stockpile => {
                    update_storage(&mut global_storage, &mut building);
                    building.updated_on_startup = true;
                }
                _ => (),
            }
        }
    }
}

fn update_storage(global_storage: &mut ResMut<GlobalStorage>, building: &mut Mut<StorageBuilding>) {
    for (resource, amount) in building.storage_data.storage.iter() {
        global_storage.update_global_storage(resource.clone(), amount.clone());
    }
}

impl StorageWithdraw for StorageBuilding {
    fn remove_from_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32> {
        println!("Stockpile: Removing from storage");
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

pub fn sys_storage_sensors(
    events: ResMut<EventQueue>,
    mut global_storage: ResMut<GlobalStorage>,
    mut collider_set: ResMut<ColliderSet>,
    mut storage_query: Query<&mut StorageBuilding>,
    mut hauler_query: Query<&mut Hauler>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let mut stockpile: Option<StorageBuilding> = None;
        let mut hauler: Option<Hauler> = None;
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);
        if let Ok(stockpile_result) = storage_query.get_mut(Entity::from_bits(entity1)) {
            match stockpile {
                None => stockpile = Some(*stockpile_result),
                _ => (),
            }
        }
        if let Ok(stockpile_result) = storage_query.get_mut(Entity::from_bits(entity2)) {
            match stockpile {
                None => stockpile = Some(*stockpile_result),
                _ => (),
            }
        }
        if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity1)) {
            match hauler {
                None => hauler = Some(*hauler_result),
                _ => (),
            }
        }
        if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity2)) {
            match hauler {
                None => hauler = Some(*hauler_result),
                _ => (),
            }
        }
        if !hauler.is_none() && !stockpile.is_none() {
            stockpile.unwrap().on_proximity_event(
                &mut global_storage,
                proximity_event.new_status,
                &mut hauler.unwrap(),
            );
        }
    }
}
