use crate::constants::enums::GameResources;
use std::collections::HashMap;

pub struct GlobalStorage {
    pub list: HashMap<GameResources, i32>,
}

impl GlobalStorage {
    pub fn new() -> Self {
        GlobalStorage {
            list: HashMap::new(),
        }
    }

    pub fn update_global_storage(&mut self, resource: GameResources, amount: i32) {
        let global_resource = self.list.get(&resource);
        match global_resource {
            Some(_global_amount) => *self.list.get_mut(&resource).unwrap() += amount,
            None => {
                self.list.insert(resource, amount.clone());
            }
        }
    }

    pub fn get_global_resouce_amount(&mut self, resource: GameResources) -> i32 {
        let global_resource = self.list.get(&resource);
        match global_resource {
            Some(global_amount) => global_amount.clone(),
            None => {
                self.list.insert(resource, 0);
                return 0;
            }
        }
    }
}
