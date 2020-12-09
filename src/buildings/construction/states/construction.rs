use crate::{
    buildings::{
        construction::{Construction, ConstructionTypes},
        warehouse::Warehouse,
    },
    characters::builder::Builder,
    managers::tasks::build::BuilderRequest,
};
use bevy::{
    ecs::{Commands, Entity, Local, Mut, Query, Res},
    prelude::{EventReader, Events},
};

pub fn state_construction_work(
    commands: &mut Commands,
    entity: &Entity,
    construction: &mut Mut<Construction>,
) {
    if construction.construction_time <= 0.0 {
        create_correct_building_component(
            commands,
            entity,
            &mut construction.construction_component,
        );
        commands.remove_one::<Construction>(*entity);
    }
}

pub fn sys_builder_request_event(
    events: Res<Events<BuilderRequest>>,
    mut event_reader: Local<EventReader<BuilderRequest>>,
    mut builder_query: Query<&mut Builder>,
) {
    for event in event_reader.iter(&events) {
        println!("Builders requested...");
        let counter = event.amount;
        for mut builder in builder_query.iter_mut() {
            if counter > 0 {
                builder.requested_construction = Some(event.construction);
                builder.movement_target = event.position;
            } else {
                break;
            }
        }
    }
}

fn create_correct_building_component(
    commands: &mut Commands,
    entity: &Entity,
    construction_component: &mut ConstructionTypes,
) {
    match construction_component {
        ConstructionTypes::WarehouseParams(capacity) => {
            commands.insert_one(*entity, Warehouse::new(capacity.clone()));
        } // ConstructionTypes::WoodcuttersHutParams(input_capacity, storage_capacity) => {
          //     commands.insert_one(
          //         *entity,
          //         WoodcuttersHut::new(input_capacity.clone(), storage_capacity.clone()),
          //     );
          // }
    }
}
