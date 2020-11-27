use crate::constants::enums::GameResources;
use std::collections::HashMap;

pub struct Storage {
    pub storage_type: StorageTypes,
}

pub enum StorageTypes {
    Stockpile,
    Warehouse,
}

#[derive(Clone)]
pub struct StorageData {
    pub max_capacity: i32,
    pub storage: HashMap<GameResources, i32>,
    pub reserved_storage: HashMap<GameResources, i32>,
    pub incoming_resources: HashMap<GameResources, i32>,
}

pub trait StorageRead {
    fn get_storage_data(&self) -> &StorageData;
    fn get_storage_data_mut(&mut self) -> &mut StorageData;
}

pub trait StorageDataRead {
    fn get_stored_amount(&self, resource: GameResources) -> i32;
    fn get_storage_usage(&self) -> i32;
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
        // storage_manager: &mut ResMut<StorageManager>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32>;
}

pub trait ResourceReservation {
    fn reserve_resource(&mut self, resource: GameResources, amount: i32) -> bool;
    fn add_incoming_resource(&mut self, resource: GameResources, amount: i32) -> bool;
}

impl StorageData {
    pub fn new(
        max_capacity: i32,
        storage: HashMap<GameResources, i32>,
        reserved_storage: HashMap<GameResources, i32>,
        incoming_resources: HashMap<GameResources, i32>,
    ) -> StorageData {
        StorageData {
            max_capacity,
            storage,
            reserved_storage,
            incoming_resources,
        }
    }
}

impl StorageDataRead for StorageData {
    fn get_stored_amount(&self, resource: GameResources) -> i32 {
        self.storage[&resource].clone()
    }

    fn get_storage_usage(&self) -> i32 {
        let mut total: i32 = 0;
        for (_, amount) in self.storage.iter() {
            total += amount;
        }
        return total;
    }
}
