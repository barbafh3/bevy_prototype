use crate::{
    buildings::{
        construction::{Construction, ConstructionTypes},
        warehouse::Warehouse,
    },
    characters::{builder::Builder, VillagerMovement},
    managers::{tasks::build::BuilderRequest, villagers::IdleVillager},
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
    mut builder_query: Query<(&IdleVillager, &mut Builder, &mut VillagerMovement)>,
) {
    for event in event_reader.iter(&events) {
        println!("Builders requested...");
        let counter = event.amount;
        for (_, mut builder, mut movement) in builder_query.iter_mut() {
            if counter > 0 {
                println!("Builder on his way...");
                builder.requested_construction = Some(event.construction);
                movement.target = event.position;
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
        ConstructionTypes::WarehouseParams(_capacity) => {
            commands.insert_one(*entity, Warehouse::new());
        } // ConstructionTypes::WoodcuttersHutParams(input_capacity, storage_capacity) => {
          //     commands.insert_one(
          //         *entity,
          //         WoodcuttersHut::new(input_capacity.clone(), storage_capacity.clone()),
          //     );
          // }
    }
}
