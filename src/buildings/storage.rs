use bevy::ecs::ResMut;

use crate::{constants::enums::GameResources, managers::storage::GlobalStorage};

use super::storage_data::StorageData;

pub trait StorageRead {
    fn get_storage_data(&self) -> &StorageData;
    fn get_storage_data_mut(&mut self) -> &mut StorageData;
}

pub trait StorageInsert {
    fn add_to_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32>;
}

pub trait StorageWithdraw {
    fn remove_from_storage(
        &mut self,
        global_storage: &mut ResMut<GlobalStorage>,
        resource: GameResources,
        amount: i32,
    ) -> Option<i32>;
}

pub trait ResourceReservation {
    fn reserve_resource(&mut self, resource: GameResources, amount: i32) -> bool;
    fn add_incoming_resource(&mut self, resource: GameResources, amount: i32) -> bool;
}
