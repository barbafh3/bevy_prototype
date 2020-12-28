use super::TaskFinished;
use crate::{
    buildings::storage_building::{get_stored_amount, StorageBuilding},
    characters::hauler::Hauler,
    constants::{enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::Query,
    ecs::Res,
    ecs::{Commands, Local},
    ecs::{Entity, ResMut},
    prelude::EventReader,
    prelude::Events,
};

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
    commands: &mut Commands,
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
        let required_haulers_raw: f32 = haul.total_resource_amount as f32 / HAULER_CAPACITY as f32;
        haul.required_haulers = required_haulers_raw.ceil() as i32;
        get_idle_haulers(haul, idle_query);
        haul.has_loaded = true;
    } else {
        let available_resource_amount = global_storage
            .get_global_resouce_amount(haul.resource_type)
            .clone();
        if available_resource_amount >= haul.total_resource_amount {
            let resources_need_to_be_hauled: bool =
                haul.total_resource_amount > 0 && haul.amount_reserved < haul.total_resource_amount;
            if resources_need_to_be_hauled {
                if haul.hauler_list.len() <= 0 {
                    if haul.required_haulers > 0 {
                        get_idle_haulers(haul, idle_query);
                    }
                } else {
                    if haul.required_haulers > 0 {
                        filter_hauler_list(haul, idle_query, storage_query);
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
        haul.hauler_list.push(entity);
    }
}

fn filter_hauler_list(
    haul: &mut Haul,
    idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>,
    storage_query: &mut Query<(Entity, &mut StorageBuilding)>,
) {
    let mut removal_list: Vec<Entity> = vec![];
    for entity in haul.hauler_list.clone().iter_mut() {
        if let Some(activated_hauler) = activate_hauler(haul, entity, idle_query, storage_query) {
            removal_list.push(activated_hauler);
        }
    }
    haul.hauler_list
        .retain(|entity| !removal_list.contains(entity));
}

fn activate_hauler(
    haul: &mut Haul,
    entity: &Entity,
    idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>,
    storage_query: &mut Query<(Entity, &mut StorageBuilding)>,
) -> Option<Entity> {
    if let Ok(result) = idle_query.get_mut(*entity) {
        let (_, _, mut hauler) = result;
        let amount_available = haul.total_resource_amount - haul.amount_reserved;
        hauler.current_resource = Some(haul.resource_type);
        hauler.resource_destination = Some(haul.resource_requester);
        match haul.resource_origin {
            Some(_origin) => hauler.resource_origin = haul.resource_origin,
            None => {
                for (entity, mut storage_building) in storage_query.iter_mut() {
                    let stored_amount =
                        get_stored_amount(&mut storage_building, hauler.current_resource.unwrap());
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
        haul.amount_reserved = hauler.amount_requested;
        haul.required_haulers -= 1;
        haul.working_haulers += 1;
        return Some(*entity);
    // removal_list.push(*entity);
    } else {
        return None;
    }
}
