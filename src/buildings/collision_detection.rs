use super::{
    construction::{on_construction_proximity_event, Construction},
    storage_building::{on_storage_building_proximity_event, StorageBuilding},
};
use crate::{
    characters::{builder::Builder, hauler::Hauler},
    managers::{events::get_entities_from_proximity_event, storage::GlobalStorage},
};
use bevy::ecs::{Entity, Mut, Query, ResMut};
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::ColliderSet};

#[derive(Debug, Eq, PartialEq)]
enum PossibleEntities {
    Hauler,
    Builder,
    Construction,
    StorageBuilding,
    None,
}

pub fn sys_filter_collision_events(
    events: ResMut<EventQueue>,
    mut global_storage: ResMut<GlobalStorage>,
    mut collider_set: ResMut<ColliderSet>,
    mut hauler_query: Query<&mut Hauler>,
    mut builder_query: Query<&mut Builder>,
    mut storage_query: Query<&mut StorageBuilding>,
    mut construction_query: Query<(Entity, &mut Construction)>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);

        let mut hauler: Option<Mut<Hauler>> = None;
        let mut builder: Option<Mut<Builder>> = None;
        let mut storage_building: Option<Mut<StorageBuilding>> = None;
        let mut construction: Option<(Entity, Mut<Construction>)> = None;

        #[allow(unused_assignments)]
        let mut entity1_type: PossibleEntities = PossibleEntities::None;
        #[allow(unused_assignments)]
        let mut entity2_type: PossibleEntities = PossibleEntities::None;

        entity1_type = filter_entity_type(
            entity1,
            &mut hauler_query,
            &mut builder_query,
            &mut storage_query,
            &mut construction_query,
        );

        entity2_type = filter_entity_type(
            entity2,
            &mut hauler_query,
            &mut builder_query,
            &mut storage_query,
            &mut construction_query,
        );

        match entity1_type {
            PossibleEntities::None => {}
            PossibleEntities::Hauler => {
                hauler = Some(hauler_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::Builder => {
                builder = Some(builder_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::StorageBuilding => {
                storage_building = Some(storage_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::Construction => {
                if let Ok((entity, local_construction)) =
                    construction_query.get_mut(Entity::from_bits(entity1))
                {
                    construction = Some((entity, local_construction))
                }
            }
        }
        match entity2_type {
            PossibleEntities::None => {}
            PossibleEntities::Hauler => {
                hauler = Some(hauler_query.get_mut(Entity::from_bits(entity2)).unwrap())
            }
            PossibleEntities::Builder => {
                builder = Some(builder_query.get_mut(Entity::from_bits(entity2)).unwrap())
            }
            PossibleEntities::StorageBuilding => {
                storage_building = Some(storage_query.get_mut(Entity::from_bits(entity2)).unwrap())
            }
            PossibleEntities::Construction => {
                construction = Some(
                    construction_query
                        .get_mut(Entity::from_bits(entity2))
                        .unwrap(),
                )
            }
        }

        let storage_and_hauler_present = !storage_building.is_none() && !hauler.is_none();
        let construction_and_hauler_present = !construction.is_none() && !hauler.is_none();
        let construction_and_builder_present = !construction.is_none() && !builder.is_none();

        if storage_and_hauler_present {
            if let (Some(mut ok_storage), Some(mut ok_hauler)) = (storage_building, hauler) {
                on_storage_building_proximity_event(
                    &mut ok_storage,
                    &mut global_storage,
                    proximity_event.new_status,
                    &mut ok_hauler,
                );
            }
        } else if construction_and_hauler_present {
            if let (Some((_, mut ok_construction)), Some(mut ok_hauler)) = (construction, hauler) {
                on_construction_proximity_event(
                    &mut ok_construction,
                    proximity_event.new_status,
                    &mut ok_hauler,
                );
            }
        } else if construction_and_builder_present {
            if let (Some(mut local_builder), Some((entity, _))) = (builder, construction) {
                // if let Some((entity, _)) = construction {
                match proximity_event.new_status {
                    bevy_rapier2d::rapier::geometry::Proximity::Intersecting => {
                        local_builder.is_inside_building = true;
                        local_builder.current_construction = Some(entity);
                        println!("Collision: Builder entered construction");
                    }
                    bevy_rapier2d::rapier::geometry::Proximity::Disjoint => {
                        local_builder.is_inside_building = false;
                        local_builder.current_construction = None;
                        println!("Collision: Builder left construction");
                    }
                    _ => {}
                }
                // }
            }
        }
    }
}

fn filter_entity_type(
    entity: u64,
    hauler_query: &mut Query<&mut Hauler>,
    builder_query: &mut Query<&mut Builder>,
    storage_query: &mut Query<&mut StorageBuilding>,
    construction_query: &mut Query<(Entity, &mut Construction)>,
) -> PossibleEntities {
    if let Ok(_result) = hauler_query.get_mut(Entity::from_bits(entity)) {
        PossibleEntities::Hauler
    } else if let Ok(_result) = builder_query.get_mut(Entity::from_bits(entity)) {
        PossibleEntities::Builder
    } else if let Ok(_result) = storage_query.get_mut(Entity::from_bits(entity)) {
        PossibleEntities::StorageBuilding
    } else if let Ok(_result) = construction_query.get_mut(Entity::from_bits(entity)) {
        PossibleEntities::Construction
    } else {
        PossibleEntities::None
    }
}
