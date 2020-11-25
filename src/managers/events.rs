use bevy::ecs::ResMut;
use bevy_rapier2d::rapier::geometry::{ColliderSet, ProximityEvent};

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
