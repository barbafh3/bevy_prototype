use bevy::ecs::{Entity, Query, ResMut};
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::geometry::{ColliderSet, Proximity},
};

use super::storage::*;
use crate::{
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::enums::GameResources,
    managers::{events::get_entities_from_proximity_event, storage::StorageManager},
};
use std::collections::HashMap;

#[derive(Clone)]
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
            HaulerStates::Loading => {
                let removal_result = self.remove_from_storage(
                    storage_manager,
                    hauler.current_resource.unwrap(),
                    hauler.amount_requested,
                );
                if let Some(amount) = removal_result {
                    hauler.take_resources(amount);
                }
            }
            _ => (),
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
pub fn sys_stockpile_sensors(
    events: ResMut<EventQueue>,
    mut storage_manager: ResMut<StorageManager>,
    mut collider_set: ResMut<ColliderSet>,
    mut warehouse_query: Query<&mut Stockpile>,
    mut hauler_query: Query<&mut Hauler>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let mut stockpile: Option<Stockpile> = None;
        let mut hauler: Option<Hauler> = None;
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);
        if let Ok(stockpile_result) = warehouse_query.get_mut(Entity::from_bits(entity1)) {
            match stockpile {
                None => stockpile = Some(stockpile_result.clone()),
                _ => (),
            }
        }
        if let Ok(stockpile_result) = warehouse_query.get_mut(Entity::from_bits(entity2)) {
            match stockpile {
                None => stockpile = Some(stockpile_result.clone()),
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

        if !hauler.is_none() && !stockpile.is_none() {
            stockpile.unwrap().on_proximity_event(
                &mut storage_manager,
                proximity_event.new_status,
                &mut hauler.unwrap(),
            );
        }
    }
}
