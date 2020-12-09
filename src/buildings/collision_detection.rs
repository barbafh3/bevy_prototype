use bevy::ecs::{Entity, Mut, Query, ResMut};
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::ColliderSet};

use crate::{
    characters::{builder::Builder, hauler::Hauler},
    managers::{events::get_entities_from_proximity_event, storage::GlobalStorage},
};

use super::{
    construction::{on_construction_proximity_event, Construction},
    stockpile::Stockpile,
    warehouse::Warehouse,
};

#[derive(Debug, Eq, PartialEq)]
enum PossibleEntities {
    Hauler,
    Builder,
    Construction,
    Stockpile,
    Warehouse,
    None,
}

pub fn sys_filter_collision_events(
    events: ResMut<EventQueue>,
    mut global_storage: ResMut<GlobalStorage>,
    mut collider_set: ResMut<ColliderSet>,
    mut hauler_query: Query<&mut Hauler>,
    mut builder_query: Query<&mut Builder>,
    mut storage_query: Query<&mut Stockpile>,
    mut warehouse_query: Query<&mut Warehouse>,
    mut construction_query: Query<(Entity, &mut Construction)>,
) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        let (entity1, entity2) =
            get_entities_from_proximity_event(proximity_event, &mut collider_set);

        let mut hauler: Option<Mut<Hauler>> = None;
        let mut builder: Option<Mut<Builder>> = None;
        let mut stockpile: Option<Mut<Stockpile>> = None;
        let mut warehouse: Option<Mut<Warehouse>> = None;
        let mut construction: Option<(Entity, Mut<Construction>)> = None;

        let mut entity1_type: PossibleEntities = PossibleEntities::None;
        let mut entity2_type: PossibleEntities = PossibleEntities::None;

        if let Ok(_result) = hauler_query.get_mut(Entity::from_bits(entity1)) {
            entity1_type = PossibleEntities::Hauler;
        } else if let Ok(_result) = builder_query.get_mut(Entity::from_bits(entity1)) {
            entity1_type = PossibleEntities::Builder;
        } else if let Ok(_result) = storage_query.get_mut(Entity::from_bits(entity1)) {
            entity1_type = PossibleEntities::Stockpile;
        } else if let Ok(_result) = warehouse_query.get_mut(Entity::from_bits(entity1)) {
            entity1_type = PossibleEntities::Warehouse;
        } else if let Ok(_result) = construction_query.get_mut(Entity::from_bits(entity1)) {
            entity1_type = PossibleEntities::Construction;
        }

        if let Ok(_result) = hauler_query.get_mut(Entity::from_bits(entity2)) {
            entity2_type = PossibleEntities::Hauler;
        } else if let Ok(_result) = builder_query.get_mut(Entity::from_bits(entity2)) {
            entity1_type = PossibleEntities::Builder;
        } else if let Ok(_result) = storage_query.get_mut(Entity::from_bits(entity2)) {
            entity2_type = PossibleEntities::Stockpile;
        } else if let Ok(_result) = warehouse_query.get_mut(Entity::from_bits(entity2)) {
            entity2_type = PossibleEntities::Warehouse;
        } else if let Ok(_result) = construction_query.get_mut(Entity::from_bits(entity2)) {
            entity2_type = PossibleEntities::Construction;
        }

        match entity1_type {
            PossibleEntities::None => {}
            PossibleEntities::Hauler => {
                hauler = Some(hauler_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::Builder => {
                builder = Some(builder_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::Stockpile => {
                stockpile = Some(storage_query.get_mut(Entity::from_bits(entity1)).unwrap())
            }
            PossibleEntities::Warehouse => {
                warehouse = Some(warehouse_query.get_mut(Entity::from_bits(entity1)).unwrap())
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
            PossibleEntities::Stockpile => {
                stockpile = Some(storage_query.get_mut(Entity::from_bits(entity2)).unwrap())
            }
            PossibleEntities::Warehouse => {
                warehouse = Some(warehouse_query.get_mut(Entity::from_bits(entity2)).unwrap())
            }
            PossibleEntities::Construction => {
                construction = Some(
                    construction_query
                        .get_mut(Entity::from_bits(entity2))
                        .unwrap(),
                )
            }
        }

        let storage_and_hauler_present = !stockpile.is_none() && !hauler.is_none();
        let warehouse_and_hauler_present = !warehouse.is_none() && !hauler.is_none();
        let construction_and_hauler_present = !construction.is_none() && !hauler.is_none();
        let construction_and_builder_present = !construction.is_none() && !builder.is_none();

        if storage_and_hauler_present {
            &mut stockpile.unwrap().on_proximity_event(
                &mut global_storage,
                proximity_event.new_status,
                &mut hauler.unwrap(),
            );
        } else if warehouse_and_hauler_present {
            &mut warehouse.unwrap().on_proximity_event(
                &mut global_storage,
                proximity_event.new_status,
                &mut hauler.unwrap(),
            );
        } else if construction_and_hauler_present {
            if let Some((_, mut construction)) = construction {
                on_construction_proximity_event(
                    &mut construction,
                    proximity_event.new_status,
                    &mut hauler.unwrap(),
                );
            }
        } else if construction_and_builder_present {
            if let Some(mut local_builder) = builder {
                if let Some((entity, _)) = construction {
                    local_builder.current_construction = Some(entity);
                }
            }
        }
    }
}
