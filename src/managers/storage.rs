use crate::constants::enums::GameResources;
use std::collections::HashMap;

pub struct StorageManager {
    global_storage: HashMap<GameResources, i32>,
}

impl StorageManager {
    pub fn new() -> StorageManager {
        let mut global_storage = HashMap::new();
        global_storage.insert(GameResources::Wood, 0);
        global_storage.insert(GameResources::Stone, 0);
        global_storage.insert(GameResources::Plank, 0);
        global_storage.insert(GameResources::StonBrick, 0);
        StorageManager { global_storage }
    }

    pub fn get_global_stored(&mut self, resource: GameResources) -> i32 {
        return self.global_storage.get(&resource).unwrap().clone();
    }

    pub fn update_global_resource(&mut self, resource: GameResources, amount: i32) {
        *self.global_storage.get_mut(&resource).unwrap() += amount;
    }
}
