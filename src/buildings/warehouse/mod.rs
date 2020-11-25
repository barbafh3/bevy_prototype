pub mod states;

use self::states::WarehouseStates;
use super::storage::*;
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::enums::GameResources,
    managers::{
        events::get_entities_from_proximity_event,
        storage::StorageManager,
        tasks::{haul::Haul, TaskManager},
    },
};
use bevy::ecs::{Entity, Mut, Query, QueryError, QuerySet, ResMut};
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::geometry::{ColliderSet, Proximity},
};
use std::collections::HashMap;

#[derive(Clone)]
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
        storage.insert(GameResources::StoneBrick, 0);
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
        &mut self,
        storage_manager: &mut ResMut<StorageManager>,
        event: Proximity,
        hauler: &mut Hauler,
    ) {
        match event {
            Proximity::Intersecting => self.on_intersect(hauler, storage_manager),
            _ => (),
        }
    }

    fn on_intersect(&mut self, hauler: &mut Hauler, storage_manager: &mut ResMut<StorageManager>) {
        println!("Warehouse: Hauler Intersect!");
        match hauler.state {
            HaulerStates::Carrying => {}
            HaulerStates::Loading => {}
            _ => (),
        }
        // let result = hauler.deliver_resource();
        // if let Some((resource, amount)) = result {
        // self.add_to_storage(storage_manager, resource, amount);
        // }
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
    fn add_to_storage(
        &mut self,
        storage_manager: &mut ResMut<StorageManager>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32> {
        let storage_data = self.get_storage_data_mut();
        if storage_data.get_storage_usage() < storage_data.max_capacity {
            let mut overflow: i32 = 0;
            let resulting_total = storage_data.get_storage_usage() + amount;
            if resulting_total <= storage_data.max_capacity {
                *storage_data.storage.get_mut(&resource).unwrap() += amount;
                storage_manager.update_global_resource(resource, amount);
            } else {
                overflow = (storage_data.storage.get(&resource).unwrap().clone() + amount)
                    - storage_data.max_capacity;
                *storage_data.storage.get_mut(&resource).unwrap() += amount - overflow;
                storage_manager.update_global_resource(resource, amount - overflow);
            }
            return Some(overflow);
        } else {
            return None;
        }
    }
}

impl StorageWithdraw for Warehouse {
    fn remove_from_storage(
        &mut self,
        storage_manager: &mut ResMut<StorageManager>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32> {
        let storage_data = self.get_storage_data_mut();
        if storage_data.get_storage_usage() > 0 {
            let storage_has_resources: bool = storage_data.get_stored_amount(resource) >= amount;
            if storage_has_resources {
                *storage_data.storage.get_mut(&resource).unwrap() -= amount;
                *storage_data.reserved_storage.get_mut(&resource).unwrap() -= amount;
                storage_manager.update_global_resource(resource, -amount);
                return Some(0);
            } else {
                let remaining_resources = storage_data.storage.get(&resource).unwrap().clone();
                *storage_data.storage.get_mut(&resource).unwrap() = 0;
                storage_manager.update_global_resource(resource, -remaining_resources);
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

pub fn sys_warehouse_sensors(
    events: ResMut<EventQueue>,
    mut storage_manager: ResMut<StorageManager>,
    mut collider_set: ResMut<ColliderSet>,
    mut warehouse_query: Query<&mut Warehouse>,
    mut hauler_query: Query<&mut Hauler>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let mut warehouse: Option<Warehouse> = None;
        let mut hauler: Option<Hauler> = None;
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);
        if let Ok(warehouse_result) = warehouse_query.get_mut(Entity::from_bits(entity1)) {
            match warehouse {
                None => warehouse = Some(warehouse_result.clone()),
                _ => (),
            }
        }
        if let Ok(warehouse_result) = warehouse_query.get_mut(Entity::from_bits(entity2)) {
            match warehouse {
                None => warehouse = Some(warehouse_result.clone()),
                _ => (),
            }
        }
        if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity1)) {
            match hauler {
                None => hauler = Some(*hauler_result),
                _ => (),
            }
        }
        if let Ok(hauler_result) = hauler_query.get_mut(Entity::from_bits(entity2)) {
            match hauler {
                None => hauler = Some(*hauler_result),
                _ => (),
            }
        }

        if !hauler.is_none() && !warehouse.is_none() {
            warehouse.unwrap().on_proximity_event(
                &mut storage_manager,
                proximity_event.new_status,
                &mut hauler.unwrap(),
            );
        }
    }
}
