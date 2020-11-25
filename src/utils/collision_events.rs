// use crate::{buildings::warehouse::Warehouse, characters::hauler::Hauler};
// use bevy::ecs::{Entity, Query, ResMut};
// use bevy_rapier2d::{
//     physics::EventQueue,
//     rapier::geometry::{ColliderSet, ProximityEvent},
// };

// pub fn sys_print_events(
//     events: ResMut<EventQueue>,
//     mut collider_set: ResMut<ColliderSet>,
//     mut query: Query<&mut Warehouse>,
//     mut query2: Query<&mut Hauler>,
// ) {
//     while let Ok(proximity_event) = events.proximity_events.pop() {
//         let (entity1, entity2) =
//             get_entities_from_proximity_event(proximity_event, &mut collider_set);
//         if let Ok(warehouse) = query.get_mut(Entity::from_bits(entity1)) {
//             warehouse.on_proximity_event(proximity_event.new_status, entity2);
//         }
//         if let Ok(warehouse) = query.get_mut(Entity::from_bits(entity2)) {
//             warehouse.on_proximity_event(proximity_event.new_status, entity1);
//         }
//     }

//     while let Ok(contact_event) = events.contact_events.pop() {
//         println!("Received contact event: {:?}", contact_event);
//     }
// }
