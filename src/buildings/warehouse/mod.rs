pub mod states;

use self::states::WarehouseStates;
use super::{
    storage::StorageInsert, storage::StorageRead, storage::StorageWithdraw,
    storage_data::StorageData, storage_data::StorageDataRead,
};
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::{enums::get_resources_list, enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
};
use bevy::ecs::{Mut, ResMut};
use bevy_rapier2d::rapier::geometry::Proximity;

pub struct Warehouse {
    pub state: WarehouseStates,
    pub storage_data: StorageData,
    pub is_sprite_set: bool,
}

impl Warehouse {
    pub fn new(max_capacity: i32) -> Warehouse {
        let warehouse = Warehouse {
            state: WarehouseStates::Active,
            storage_data: StorageData::new(
                max_capacity,
                get_resources_list(),
                get_resources_list(),
                get_resources_list(),
            ),
            is_sprite_set: false,
        };
        return warehouse;
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
        match self.state {
            WarehouseStates::Active => self.check_hauler_state(global_storage, hauler),
            _ => (),
        }
    }

    fn check_hauler_state(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        hauler: &mut Mut<Hauler>,
    ) {
        match hauler.state {
            HaulerStates::Loading => self.deliver_resources(global_storage, hauler),
            HaulerStates::Carrying => self.take_resources(),
            _ => (),
        }
    }

    fn deliver_resources(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        hauler: &mut Mut<Hauler>,
    ) {
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

    fn take_resources(&mut self) {}
}

impl StorageInsert for Warehouse {
    fn add_to_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32> {
        let total_amount_after_adding = self.storage_data.storage[resource] + amount;
        if total_amount_after_adding <= self.storage_data.max_capacity {
            self.storage_data.storage[resource] += amount;
            Some(0)
        } else {
            let amount_possible_to_add =
                amount - (total_amount_after_adding - self.storage_data.max_capacity);
            let amount_remaining_on_hauler = amount - amount_possible_to_add;
            if amount_possible_to_add > 0 {
                self.storage_data.storage[resource] += amount;
                Some(amount_remaining_on_hauler)
            } else {
                None
            }
        }
    }
}

impl StorageWithdraw for Warehouse {
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

impl StorageRead for Warehouse {
    fn get_storage_data_mut(&mut self) -> &mut StorageData {
        return &mut self.storage_data;
    }
    fn get_storage_data(&self) -> &StorageData {
        return &self.storage_data;
    }
}
