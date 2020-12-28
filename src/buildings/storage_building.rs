use crate::{
    characters::hauler::{states::HaulerStates, Hauler},
    constants::{enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
};
use bevy::ecs::{Mut, Query, ResMut};
use bevy_rapier2d::rapier::geometry::Proximity;
use enum_map::EnumMap;

#[derive(Copy, Clone)]
pub struct StorageBuilding {
    pub max_capacity: i32,
    pub storage: EnumMap<GameResources, i32>,
    pub reserved_storage: EnumMap<GameResources, i32>,
    pub incoming_resources: EnumMap<GameResources, i32>,
    pub updated_on_startup: bool,
}

impl StorageBuilding {
    pub fn new(
        max_capacity: i32,
        storage: EnumMap<GameResources, i32>,
        reserved_storage: EnumMap<GameResources, i32>,
        incoming_resources: EnumMap<GameResources, i32>,
    ) -> StorageBuilding {
        StorageBuilding {
            max_capacity,
            storage,
            reserved_storage,
            incoming_resources,
            updated_on_startup: false,
        }
    }
}

pub fn get_stored_amount(
    storage_building: &mut Mut<StorageBuilding>,
    resource: GameResources,
) -> i32 {
    storage_building.storage[resource].clone()
}

fn get_storage_usage(storage_building: &StorageBuilding) -> i32 {
    let mut total: i32 = 0;
    for (_, amount) in storage_building.storage.iter() {
        total += amount;
    }
    return total;
}

pub fn on_storage_building_proximity_event(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    event: Proximity,
    hauler: &mut Mut<Hauler>,
) {
    match event {
        Proximity::Intersecting => {
            on_storage_building_intersect(storage_building, global_storage, hauler)
        }
        _ => (),
    }
}

fn on_storage_building_intersect(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    hauler: &mut Mut<Hauler>,
) {
    println!("StorageBuilding: Proximity!");
    match hauler.state {
        HaulerStates::Loading => {
            let removal_result = remove_from_storage_building(
                storage_building,
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

fn check_hauler_state(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    hauler: &mut Mut<Hauler>,
) {
    match hauler.state {
        HaulerStates::Loading => deliver_resources(storage_building, global_storage, hauler),
        HaulerStates::Carrying => take_resources(),
        _ => (),
    }
}

pub fn add_to_storage_building(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    resource: GameResources,
    amount: i32,
) -> Option<i32> {
    let total_amount_after_adding = storage_building.storage[resource] + amount;
    if total_amount_after_adding <= storage_building.max_capacity {
        storage_building.storage[resource] += amount;
        global_storage.update_global_storage(resource, amount);
        Some(0)
    } else {
        let amount_possible_to_add =
            amount - (total_amount_after_adding - storage_building.max_capacity);
        let amount_remaining_on_hauler = amount - amount_possible_to_add;
        if amount_possible_to_add > 0 {
            storage_building.storage[resource] += amount_possible_to_add;
            global_storage.update_global_storage(resource, amount_possible_to_add);
            Some(amount_remaining_on_hauler)
        } else {
            None
        }
    }
}

pub fn remove_from_storage_building(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    resource: GameResources,
    amount: i32,
) -> Option<i32> {
    if get_storage_usage(storage_building) > 0 {
        let storage_has_resources: bool = storage_building.storage[resource] >= amount;
        if storage_has_resources {
            storage_building.storage[resource] -= amount;
            storage_building.reserved_storage[resource] -= amount;
            global_storage.update_global_storage(resource, -amount);
            return Some(0);
        } else {
            let remaining_resources = storage_building.storage[resource].clone();
            storage_building.storage[resource] = 0;
            global_storage.update_global_storage(resource, -remaining_resources);
            let amount_not_removed = amount - remaining_resources;
            return Some(amount_not_removed);
        }
    } else {
        return None;
    }
}

pub fn deliver_resources(
    storage_building: &mut StorageBuilding,
    global_storage: &mut ResMut<GlobalStorage>,
    hauler: &mut Mut<Hauler>,
) {
    let removal_result = remove_from_storage_building(
        storage_building,
        global_storage,
        hauler.current_resource.unwrap(),
        hauler.amount_requested,
    );
    if let Some(remainder) = removal_result {
        println!("Stockpile: Remainder = {}", remainder);
        if remainder == 0 {
            hauler.capacity = HAULER_CAPACITY;
        } else {
            hauler.capacity = HAULER_CAPACITY - remainder;
        }
    }
}

pub fn take_resources() {}

pub fn sys_update_building_storage(
    mut global_storage: ResMut<GlobalStorage>,
    mut query: Query<&mut StorageBuilding>,
) {
    for mut building in query.iter_mut() {
        if !building.updated_on_startup {
            for (resource, amount) in building.storage.iter() {
                global_storage.update_global_storage(resource.clone(), amount.clone());
            }
            building.updated_on_startup = true
        }
    }
}
