use super::TaskFinished;
use crate::{
    buildings::storage::StorageBuilding,
    buildings::storage::StorageDataRead,
    buildings::storage::StorageRead,
    characters::hauler::states::HaulerStates,
    characters::hauler::Hauler,
    constants::{enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::Mut,
    ecs::Query,
    ecs::Res,
    ecs::{Commands, Local},
    ecs::{Entity, ResMut},
    prelude::EventReader,
    prelude::Events,
};
use std::collections::HashMap;

pub struct Haul {
    has_loaded: bool,
    total_resource_amount: i32,
    amount_reserved: i32,
    resource_type: GameResources,
    resource_requester: Entity,
    resource_origin: Option<Entity>,
    required_haulers: i32,
    working_haulers: i32,
    hauler_list: Vec<Entity>,
}

impl Haul {
    pub fn new(
        resource_type: GameResources,
        total_resource_amount: i32,
        resource_requester: Entity,
        resource_origin: Option<Entity>,
    ) -> Haul {
        Haul {
            has_loaded: false,
            resource_type,
            total_resource_amount,
            amount_reserved: 0,
            resource_requester,
            resource_origin,
            required_haulers: 0,
            working_haulers: 0,
            hauler_list: vec![],
        }
    }
}

pub fn sys_run_haul_tasks(
    mut global_storage: ResMut<GlobalStorage>,
    mut events: ResMut<Events<TaskFinished>>,
    mut haul_query: Query<(Entity, &mut Haul)>,
    mut idle_query: Query<(Entity, &IdleVillager, &mut Hauler)>,
    mut storage_query: Query<(Entity, &mut StorageBuilding)>,
) {
    for (entity, mut haul) in haul_query.iter_mut() {
        run_haul(
            &mut haul,
            entity,
            &mut events,
            &mut global_storage,
            &mut idle_query,
            &mut storage_query,
        );
    }
}

pub fn sys_close_haul_tasks(
    mut commands: Commands,
    mut my_event_reader: Local<EventReader<TaskFinished>>,
    events: Res<Events<TaskFinished>>,
) {
    for event in my_event_reader.iter(&events) {
        commands.despawn(event.task);
    }
}

fn run_haul(
    haul: &mut Haul,
    task_entity: Entity,
    events: &mut ResMut<Events<TaskFinished>>,
    global_storage: &mut ResMut<GlobalStorage>,
    idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>,
    storage_query: &mut Query<(Entity, &mut StorageBuilding)>,
) {
    if !haul.has_loaded {
        println!("Haul: Starting...");
        println!("Haul: Requested amount: {}", haul.total_resource_amount);
        let required_haulers_raw: f32 = haul.total_resource_amount as f32 / HAULER_CAPACITY as f32;
        haul.required_haulers = required_haulers_raw.ceil() as i32;
        get_idle_haulers(haul, idle_query);
        haul.has_loaded = true;
        println!("Haul: Started!");
    } else {
        let available_resource_amount = global_storage
            .get_global_resouce_amount(haul.resource_type)
            .clone();
        if available_resource_amount >= haul.total_resource_amount {
            if haul.total_resource_amount > 0 && haul.amount_reserved < haul.total_resource_amount {
                if haul.hauler_list.len() <= 0 {
                    if haul.required_haulers > 0 {
                        get_idle_haulers(haul, idle_query);
                    }
                } else {
                    if haul.required_haulers > 0 {
                        let mut removal_list: Vec<Entity> = vec![];
                        for entity in haul.hauler_list.iter() {
                            if let Ok(result) = idle_query.get_mut(*entity) {
                                let (_, _, mut hauler) = result;
                                let amount_available =
                                    haul.total_resource_amount - haul.amount_reserved;
                                hauler.current_resource = Some(haul.resource_type);
                                hauler.resource_destination = Some(haul.resource_requester);
                                match haul.resource_origin {
                                    Some(_origin) => hauler.resource_origin = haul.resource_origin,
                                    None => {
                                        for (entity, storage_building) in storage_query.iter_mut() {
                                            let stored_amount = storage_building
                                                .get_storage_data()
                                                .get_stored_amount(
                                                    hauler.current_resource.unwrap(),
                                                );
                                            if stored_amount >= 0 {
                                                hauler.resource_origin = Some(entity);
                                            }
                                        }
                                    }
                                }
                                if amount_available > HAULER_CAPACITY {
                                    hauler.amount_requested = HAULER_CAPACITY
                                } else {
                                    hauler.amount_requested = amount_available;
                                }
                                hauler.state = HaulerStates::Loading;
                                println!(
                                    "Haul: Hauler amount: {}",
                                    hauler.amount_requested.clone()
                                );
                                haul.amount_reserved = hauler.amount_requested;
                                haul.required_haulers -= 1;
                                haul.working_haulers += 1;
                                removal_list.push(*entity);
                            }
                        }
                        haul.hauler_list
                            .retain(|entity| !removal_list.contains(entity));
                    }
                }
            }
            if haul.total_resource_amount <= 0 {
                events.send(TaskFinished { task: task_entity });
            }
        }
    }
}

fn get_idle_haulers(haul: &mut Haul, idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>) {
    for (entity, _, _) in idle_query.iter_mut() {
        println!("Haul: Idle hauler found!");
        haul.hauler_list.push(entity);
    }
}

fn activate_hauler(
    haul: &Haul,
    entity: &Entity,
    hauler: &mut Hauler,
    storage_query: &mut Query<(Entity, &mut StorageBuilding)>,
) -> i32 {
    return hauler.amount_requested;
}
