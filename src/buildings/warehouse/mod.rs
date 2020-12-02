pub mod states;

use self::states::WarehouseStates;
use super::{
    building::ConstructionWork, storage::StorageInsert, storage::StorageRead,
    storage::StorageWithdraw, storage_data::StorageData, storage_data::StorageDataRead,
};
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::{enums::get_resources_list, enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
};
use bevy::ecs::{Mut, ResMut};
use bevy_rapier2d::rapier::geometry::Proximity;
use enum_map::EnumMap;

pub struct Warehouse {
    pub state: WarehouseStates,
    pub storage_data: StorageData,
    pub required_resources: EnumMap<GameResources, i32>,
    pub construction_time: f32,
    pub has_requested_resources: bool,
    pub warehouse_sprite_added: bool,
    pub is_position_set: bool,
}

impl Warehouse {
    pub fn new(
        max_capacity: i32,
        required_resources: EnumMap<GameResources, i32>,
        construction_time: f32,
    ) -> Warehouse {
        let warehouse = Warehouse {
            state: WarehouseStates::Placing,
            storage_data: StorageData::new(
                max_capacity,
                get_resources_list(),
                get_resources_list(),
                get_resources_list(),
            ),
            required_resources,
            construction_time,
            has_requested_resources: false,
            warehouse_sprite_added: false,
            is_position_set: false,
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
            WarehouseStates::Loading => self.receive_resources(global_storage, hauler),
            WarehouseStates::Idle => self.check_hauler_state(global_storage, hauler),
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
            HaulerStates::Carrying => self.receive_resources(global_storage, hauler),
            _ => (),
        }
    }

    fn receive_resources(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        hauler: &mut Mut<Hauler>,
    ) {
        let adding_result = self.add_to_storage(
            global_storage,
            hauler.current_resource.unwrap(),
            hauler.capacity,
        );
        if let Some(remainder) = adding_result {
            if self.required_resources[hauler.current_resource.unwrap()] > 0 {
                self.required_resources[hauler.current_resource.unwrap()] -=
                    hauler.capacity - remainder;
            }
            hauler.deliver_resource();
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
}

impl ConstructionWork for Warehouse {
    fn do_construction_work(&mut self) {
        todo!()
    }
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
            global_storage.update_global_storage(resource, amount);
            Some(0)
        } else {
            let amount_possible_to_add =
                amount - (total_amount_after_adding - self.storage_data.max_capacity);
            let amount_remaining_on_hauler = amount - amount_possible_to_add;
            if amount_possible_to_add > 0 {
                self.storage_data.storage[resource] += amount_possible_to_add;
                global_storage.update_global_storage(resource, amount_possible_to_add);
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
