use enum_map::EnumMap;

use crate::constants::enums::GameResources;

// #[derive(Clone)]
// pub struct StorageData {
//     pub max_capacity: i32,
//     pub storage: HashMap<GameResources, i32>,
//     pub reserved_storage: HashMap<GameResources, i32>,
//     pub incoming_resources: HashMap<GameResources, i32>,
// }

#[derive(Copy, Clone)]
pub struct StorageData {
    pub max_capacity: i32,
    pub storage: EnumMap<GameResources, i32>,
    pub reserved_storage: EnumMap<GameResources, i32>,
    pub incoming_resources: EnumMap<GameResources, i32>,
}

pub trait StorageDataRead {
    fn get_stored_amount(&self, resource: GameResources) -> i32;
    fn get_storage_usage(&self) -> i32;
}

impl StorageData {
    pub fn new(
        max_capacity: i32,
        storage: EnumMap<GameResources, i32>,
        reserved_storage: EnumMap<GameResources, i32>,
        incoming_resources: EnumMap<GameResources, i32>,
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
        self.storage[resource].clone()
    }

    fn get_storage_usage(&self) -> i32 {
        let mut total: i32 = 0;
        for (_, amount) in self.storage.iter() {
            total += amount;
        }
        return total;
    }
}
