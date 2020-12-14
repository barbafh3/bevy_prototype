#[derive(Default)]
pub struct Stockpile;

// impl Stockpile {
//     pub fn new(max_capacity: i32, desired_storage: EnumMap<GameResources, i32>) -> Self {
//         let building = Stockpile {
//             storage_data: StorageData::new(
//                 max_capacity,
//                 desired_storage,
//                 get_resources_list(),
//                 get_resources_list(),
//             ),
//             updated_on_startup: false,
//         };
//         return building;
//     }
// }

// impl StorageRead for Stockpile {
//     fn get_storage_data_mut(&mut self) -> &mut StorageData {
//         return &mut self.storage_data;
//     }
//     fn get_storage_data(&self) -> &StorageData {
//         return &self.storage_data;
//     }
// }

// pub fn sys_update_stockpile_storage(
//     mut global_storage: ResMut<GlobalStorage>,
//     mut query: Query<&mut Stockpile>,
// ) {
//     for building in query.iter_mut() {
//         if !building.updated_on_startup {
//             for (resource, amount) in building.storage_data.storage.iter() {
//                 global_storage.update_global_storage(resource.clone(), amount.clone());
//             }
//         }
//     }
// }

// pub fn on_stockpile_proximity_event(
//     stockpile: &mut Stockpile,
//     global_storage: &mut ResMut<GlobalStorage>,
//     event: Proximity,
//     hauler: &mut Mut<Hauler>,
// ) {
//     match event {
//         Proximity::Intersecting => on_stockpile_intersect(stockpile, global_storage, hauler),
//         _ => (),
//     }
// }

// fn on_stockpile_intersect(
//     stockpile: &mut Stockpile,
//     global_storage: &mut ResMut<GlobalStorage>,
//     hauler: &mut Mut<Hauler>,
// ) {
//     match hauler.state {
//         HaulerStates::Loading => {
//             let removal_result = remove_from_stockpile_storage(
//                 stockpile,
//                 global_storage,
//                 hauler.current_resource.unwrap(),
//                 hauler.amount_requested,
//             );
//             if let Some(remainder) = removal_result {
//                 if remainder == 0 {
//                     hauler.capacity = HAULER_CAPACITY;
//                 } else {
//                     hauler.capacity = HAULER_CAPACITY - remainder;
//                 }
//             }
//         }
//         _ => (),
//     }
// }

// pub fn remove_from_stockpile_storage(
//     stockpile: &mut Stockpile,
//     global_storage: &mut ResMut<GlobalStorage>,
//     resource: GameResources,
//     amount: i32,
// ) -> Option<i32> {
//     let storage_data = stockpile.get_storage_data_mut();
//     if storage_data.get_storage_usage() > 0 {
//         let storage_has_resources: bool = storage_data.get_stored_amount(resource) >= amount;
//         if storage_has_resources {
//             storage_data.storage[resource] -= amount;
//             storage_data.reserved_storage[resource] -= amount;
//             global_storage.update_global_storage(resource, -amount);
//             return Some(0);
//         } else {
//             let remaining_resources = storage_data.storage[resource].clone();
//             storage_data.storage[resource] = 0;
//             global_storage.update_global_storage(resource, -remaining_resources);
//             let amount_not_removed = amount - remaining_resources;
//             return Some(amount_not_removed);
//         }
//     } else {
//         return None;
//     }
// }
