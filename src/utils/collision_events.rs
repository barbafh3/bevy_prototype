use bevy::ecs::{Entity, Query, ResMut};
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::geometry::{ColliderSet, ProximityEvent},
};

use crate::{
    buildings::warehouse::Warehouse, characters::player::Player, managers::tasks::TaskManager,
};

pub fn sys_print_events(
    events: ResMut<EventQueue>,
    mut collider_set: ResMut<ColliderSet>,
    mut task_manager: ResMut<TaskManager>,
    mut query: Query<&mut Player>,
    mut query2: Query<&mut Warehouse>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);
        if let Ok(_player) = query.get_mut(Entity::from_bits(entity1)) {
            // println!("{}", player.on_proximity_event(proximity_event.new_status));
        } else if let Ok(warehouse) = query2.get_mut(Entity::from_bits(entity1)) {
            println!(
                "{}",
                warehouse.on_proximity_event(proximity_event.new_status, &mut task_manager)
            );
        }
        if let Ok(_player) = query.get_mut(Entity::from_bits(entity2)) {
            // println!("{}", player.on_proximity_event(proximity_event.new_status));
        } else if let Ok(warehouse) = query2.get_mut(Entity::from_bits(entity2)) {
            println!(
                "{}",
                warehouse.on_proximity_event(proximity_event.new_status, &mut task_manager)
            );
        }
        // println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}

pub fn get_entities_from_proximity_event(
    proximity_event: ProximityEvent,
    collider_set: &mut ResMut<ColliderSet>,
) -> (u64, u64) {
    return (
        collider_set
            .get_mut(proximity_event.collider1)
            .unwrap()
            .user_data as u64,
        collider_set
            .get_mut(proximity_event.collider2)
            .unwrap()
            .user_data as u64,
    );
}
