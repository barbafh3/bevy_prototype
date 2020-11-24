pub mod states;

use self::states::WarehouseStates;
use super::storage::*;
use crate::{
    constants::enums::GameResources,
    managers::tasks::{haul::Haul, TaskManager},
};
use bevy::ecs::ResMut;
use bevy_rapier2d::rapier::geometry::Proximity;
use std::collections::HashMap;

pub struct Warehouse {
    pub state: WarehouseStates,
    pub required_resources: HashMap<GameResources, i32>,
    pub construction_time: f32,
    pub warehouse_sprite_added: bool,
    pub is_position_set: bool,
    storage_data: StorageData,
}

impl Warehouse {
    pub fn new(max_capacity: i32, required_resources: HashMap<GameResources, i32>) -> Warehouse {
        let mut storage: HashMap<GameResources, i32> = HashMap::new();
        storage.insert(GameResources::Wood, 0);
        storage.insert(GameResources::Stone, 0);
        storage.insert(GameResources::Plank, 0);
        storage.insert(GameResources::StonBrick, 0);
        let warehouse = Warehouse {
            state: WarehouseStates::Placing,
            required_resources,
            construction_time: 10.0,
            warehouse_sprite_added: false,
            is_position_set: false,
            storage_data: StorageData::new(
                max_capacity,
                storage.clone(),
                storage.clone(),
                storage.clone(),
            ),
        };
        return warehouse;
    }

    pub fn on_proximity_event(
        &self,
        event: Proximity,
        task_manager: &mut ResMut<TaskManager>,
    ) -> String {
        let mut output = "A body ".to_string();
        match event {
            Proximity::Intersecting => self.on_intersect(task_manager),
            Proximity::Disjoint => output.push_str("just left the area"),
            Proximity::WithinMargin => output.push_str("is nearby"),
        }
        return output;
    }

    fn on_intersect(&self, task_manager: &mut ResMut<TaskManager>) {
        let haul = Haul::new(9.0, 1.0);
        task_manager.register_task(haul);
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

impl StorageInsert for Warehouse {
    fn add_to_storage(&mut self, resource: GameResources, amount: i32) -> Option<i32> {
        let storage_data = self.get_storage_data_mut();
        if storage_data.get_storage_usage() < storage_data.max_capacity {
            let mut overflow: i32 = 0;
            let resulting_total = storage_data.get_storage_usage() + amount;
            if resulting_total <= storage_data.max_capacity {
                *storage_data.storage.get_mut(&resource).unwrap() += amount;
            // TODO: Call storage manager to update values
            } else {
                overflow = (storage_data.storage.get(&resource).unwrap().clone() + amount)
                    - storage_data.max_capacity;
                *storage_data.storage.get_mut(&resource).unwrap() += amount - overflow;
                // TODO: Call storage manager to update values
            }
            return Some(overflow);
        } else {
            return None;
        }
    }
}

impl StorageWithdraw for Warehouse {
    fn remove_from_storage(&mut self, resource: GameResources, amount: i32) -> Option<i32> {
        let storage_data = self.get_storage_data_mut();
        if storage_data.get_storage_usage() > 0 {
            let storage_has_resources: bool = storage_data.get_stored_amount(resource) >= amount;
            if storage_has_resources {
                *storage_data.storage.get_mut(&resource).unwrap() -= amount;
                *storage_data.reserved_storage.get_mut(&resource).unwrap() -= amount;
                // TODO: Call storage manager to update values
                return Some(0);
            } else {
                let remaining_resources = storage_data.storage.get(&resource).unwrap().clone();
                *storage_data.storage.get_mut(&resource).unwrap() = 0;
                // TODO: Call storage manager to update values
                let amount_not_removed = amount - remaining_resources;
                return Some(amount_not_removed);
            }
        } else {
            return None;
        }
    }
}

impl ResourceReservation for Warehouse {
    fn reserve_resource(&mut self, resource: GameResources, amount: i32) -> bool {
        let storage_data = self.get_storage_data_mut();
        let storage_can_reserve_resource = *storage_data.storage.get(&resource).unwrap() >= amount;
        if storage_can_reserve_resource {
            *storage_data.reserved_storage.get_mut(&resource).unwrap() += amount;
            return true;
        } else {
            return false;
        }
    }

    fn add_incoming_resource(&mut self, resource: GameResources, amount: i32) -> bool {
        let storage_data = self.get_storage_data_mut();
        let storage_has_room: bool =
            (storage_data.get_storage_usage() + amount) <= storage_data.max_capacity;
        if storage_has_room {
            *storage_data.incoming_resources.get_mut(&resource).unwrap() += amount;
            return true;
        } else {
            return false;
        }
    }
}
