#![feature(type_name_of_val)]
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

use crate::characters::hauler::Hauler;

pub struct VillagerManager {
    villagers: HashMap<i32, Box<dyn IndexedVillager + Send + Sync>>,
}

lazy_static! {
    pub static ref VILLAGER_MANAGER: Mutex<VillagerManager> = Mutex::new(VillagerManager::new());
}

impl VillagerManager {
    pub fn new() -> VillagerManager {
        VillagerManager {
            villagers: HashMap::new(),
        }
    }

    pub fn register_villager<T: IndexedVillager + 'static + Send + Sync>(
        &mut self,
        mut villager: T,
    ) {
        self.register_villager_recursive(villager);
    }

    fn register_villager_recursive<T: IndexedVillager + 'static + Send + Sync>(
        &mut self,
        mut villager: T,
    ) {
        let key = rand::random::<i32>().abs();
        if self.villagers.contains_key(&key) {
            self.register_villager_recursive(villager);
        } else {
            villager.set_villager_index(key);
            self.villagers
                .insert(villager.get_villager_index(), Box::new(villager));
        }
    }

    // fn request_idle_villager<T: IndexedVillager + 'static + Send + Sync>(
    //     &mut self,
    //     villager_type: T,
    // ) {
    //     let selected_villager: Option<T> = None;
    //     for (index, villager) in self.villagers.iter() {
    //         let is_villager_type_equal =
    //             type_of_villager(villager_type) == type_of_villager(*villager);
    //         if is_villager_type_equal && villager.is_idle() {}
    //     }
    // }

    fn request_idle_villager(&mut self, hauler: Hauler) {
        let selected_villager: Option<Hauler> = None;
        for (index, villager) in self.villagers.iter() {
            let is_villager_type_equal = false;
            if is_villager_type_equal && villager.is_idle() {}
        }
    }
}

// fn unbox<T>(value: Box<T>) -> T {
//     *value
// }

// pub fn type_of_villager<T: Any>(_: T) -> &'static str {
//     type_name()
// }

pub trait IndexedVillager {
    fn get_villager_index(&self) -> i32;
    fn set_villager_index(&mut self, index: i32);
    fn is_idle(&self) -> bool;
    fn set_status(&mut self, idle: bool);
}
