use crate::{
    buildings::{
        construction::{Construction, ConstructionTypes},
        warehouse::Warehouse,
        woodcuttershut::WoodcuttersHut,
    },
    characters::builder::Builder,
    managers::tasks::build::BuilderRequest,
};
use bevy::{
    core::Time,
    ecs::{Commands, Entity, Local, Mut, Query, Res},
    prelude::{EventReader, Events},
};
use bevy_rapier2d::physics::EventQueue;

pub fn state_construction_work(
    time: &Res<Time>,
    commands: &mut Commands,
    entity: &Entity,
    construction: &mut Mut<Construction>,
) {
    construction.construction_time = run_construction_tick(&construction, time.delta_seconds);
    if construction.construction_time <= 0.0 {
        match construction.construction_component {
            ConstructionTypes::WarehouseParams(capacity) => {
                commands.insert_one(*entity, Warehouse::new(capacity));
            }
            ConstructionTypes::WoodcuttersHutParams(input_capacity, storage_capacity) => {
                commands.insert_one(
                    *entity,
                    WoodcuttersHut::new(input_capacity, storage_capacity),
                );
            }
        }
        commands.remove_one::<Construction>(*entity);
    }
}

fn run_construction_tick(construction: &Mut<Construction>, delta: f32) -> f32 {
    if construction.construction_time > 0.0 {
        return construction.construction_time - delta;
    } else {
        return construction.construction_time;
    }
}

pub fn sys_builder_request_event(
    events: Res<Events<BuilderRequest>>,
    mut event_reader: Local<EventReader<BuilderRequest>>,
    mut builder_query: Query<&mut Builder>,
) {
    for event in event_reader.iter(&events) {
        let counter = event.amount;
        for mut builder in builder_query.iter_mut() {
            if counter > 0 {
                builder.requested_construction = event.construction;
            } else {
                break;
            }
        }
    }
}
