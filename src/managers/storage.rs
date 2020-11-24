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
}
