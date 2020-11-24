use super::storage::*;
use crate::constants::enums::GameResources;
use std::collections::HashMap;

pub struct Stockpile {
    storage_data: StorageData,
}

impl Stockpile {
    pub fn new(
        max_capacity: i32,
        storage: HashMap<GameResources, i32>,
        reserved_storage: HashMap<GameResources, i32>,
        incoming_resources: HashMap<GameResources, i32>,
    ) -> Stockpile {
        Stockpile {
            storage_data: StorageData::new(
                max_capacity,
                storage,
                reserved_storage,
                incoming_resources,
            ),
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

impl StorageWithdraw for Stockpile {
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
