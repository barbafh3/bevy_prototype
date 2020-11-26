use super::TaskFinished;
use crate::{
    characters::hauler::Hauler,
    constants::{enums::GameResources, tasks::HAULER_CAPACITY},
    managers::storage::GlobalStorage,
    managers::villagers::IdleVillager,
};
use bevy::{
    core::Time,
    ecs::Query,
    ecs::Res,
    ecs::{Commands, Local},
    ecs::{Entity, ResMut},
    prelude::EventReader,
    prelude::Events,
};
use std::collections::HashMap;

pub struct Haul {
    task_index: i32,
    priority: f32,
    weight: f32,
    timer: f32,
    has_loaded: bool,
    total_resource_amount: i32,
    amount_reserved: i32,
    resource_type: GameResources,
    resource_requester: Entity,
    resource_origin: Option<Entity>,
    required_haulers: i32,
    working_haulers: i32,
    hauler_list: HashMap<Entity, Hauler>,
}

impl Haul {
    pub fn new(
        priority: f32,
        weight: f32,
        resource_type: GameResources,
        total_resource_amount: i32,
        resource_requester: Entity,
        resource_origin: Option<Entity>,
        hauler_list: HashMap<Entity, Hauler>,
    ) -> Haul {
        Haul {
            task_index: 0,
            priority,
            weight,
            has_loaded: false,
            timer: 3.0,
            resource_type,
            total_resource_amount,
            amount_reserved: 0,
            resource_requester,
            resource_origin,
            required_haulers: 0,
            working_haulers: 0,
            hauler_list,
        }
    }

    // pub fn run_haul(
    //     &mut self,
    //     task_entity: Entity,
    //     delta: f32,
    //     events: &mut ResMut<Events<TaskFinished>>,
    //     global_storage: &Res<GlobalStorage>,
    //     idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>,
    // ) {
    //     if !self.has_loaded {
    //         let required_haulers_raw: f32 =
    //             self.total_resource_amount as f32 / HAULER_CAPACITY as f32;
    //         self.required_haulers = required_haulers_raw.ceil() as i32;
    //         self.get_idle_haulers(idle_query);
    //         self.has_loaded = true;
    //     }
    //     let available_resource_amount =
    //         global_storage.get_global_resouce_amount(self.resource_type);
    //     if available_resource_amount >= self.total_resource_amount {
    //         if self.total_resource_amount > 0 && self.amount_reserved < self.total_resource_amount {
    //             if self.hauler_list.len() <= 0 {
    //                 if self.required_haulers > 0 {
    //                     self.get_idle_haulers(idle_query);
    //                 }
    //             } else {
    //                 for (entity, hauler) in self.hauler_list.iter_mut() {
    //                     self.activate_hauler(entity, hauler);
    //                 }
    //             }
    //         }
    //         if self.total_resource_amount <= 0 {
    //             events.send(TaskFinished { task: task_entity });
    //         }
    //     }
    // }

    // fn get_idle_haulers(&mut self, idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>) {
    //     for (entity, _, hauler) in idle_query.iter_mut() {
    //         let hauler_list_length: i32 = self.hauler_list.len() as i32;
    //         if hauler_list_length < self.required_haulers {
    //             self.hauler_list.insert(entity, *hauler);
    //         }
    //     }
    // }

    // fn activate_hauler(&mut self, entity: &Entity, hauler: &mut Hauler) {
    //     let amount_available = self.total_resource_amount - self.amount_reserved;
    //     hauler.current_resource = Some(self.resource_type);
    //     hauler.resource_destination = Some(self.resource_requester);
    //     match self.resource_origin {
    //         Some(origin) => hauler.resource_origin = self.resource_origin,
    //         None => {
    //             // Gets a new origin for the resource (Warehouse, Stockpile, etc)
    //         }
    //     }
    //     if amount_available > HAULER_CAPACITY {
    //         hauler.amount_requested = HAULER_CAPACITY
    //     } else {
    //         hauler.amount_requested = amount_available;
    //     }
    //     self.amount_reserved += hauler.amount_requested;
    //     self.required_haulers -= 1;
    //     self.hauler_list.remove(entity);
    // }
}

pub fn sys_run_haul_tasks(
    time: Res<Time>,
    mut global_storage: ResMut<GlobalStorage>,
    mut events: ResMut<Events<TaskFinished>>,
    mut haul_query: Query<(Entity, &mut Haul)>,
    mut idle_query: Query<(Entity, &IdleVillager, &mut Hauler)>,
) {
    for (entity, mut haul) in haul_query.iter_mut() {
        run_haul(
            &mut haul,
            entity,
            time.delta_seconds,
            &mut events,
            &mut global_storage,
            &mut idle_query,
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
    mut haul: &mut Haul,
    task_entity: Entity,
    delta: f32,
    events: &mut ResMut<Events<TaskFinished>>,
    global_storage: &mut ResMut<GlobalStorage>,
    idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>,
) {
    if !haul.has_loaded {
        let required_haulers_raw: f32 = haul.total_resource_amount as f32 / HAULER_CAPACITY as f32;
        haul.required_haulers = required_haulers_raw.ceil() as i32;
        get_idle_haulers(haul, idle_query);
        haul.has_loaded = true;
    }
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
                for (entity, hauler) in haul.hauler_list.clone().iter_mut() {
                    activate_hauler(&mut haul, entity, hauler);
                }
            }
        }
        if haul.total_resource_amount <= 0 {
            events.send(TaskFinished { task: task_entity });
        }
    }
}

fn get_idle_haulers(haul: &mut Haul, idle_query: &mut Query<(Entity, &IdleVillager, &mut Hauler)>) {
    for (entity, _, hauler) in idle_query.iter_mut() {
        let hauler_list_length: i32 = haul.hauler_list.len() as i32;
        if hauler_list_length < haul.required_haulers {
            haul.hauler_list.insert(entity, *hauler);
        }
    }
}

fn activate_hauler(haul: &mut Haul, entity: &Entity, hauler: &mut Hauler) {
    let amount_available = haul.total_resource_amount - haul.amount_reserved;
    hauler.current_resource = Some(haul.resource_type);
    hauler.resource_destination = Some(haul.resource_requester);
    match haul.resource_origin {
        Some(origin) => hauler.resource_origin = haul.resource_origin,
        None => {
            // Gets a new origin for the resource (Warehouse, Stockpile, etc)
        }
    }
    if amount_available > HAULER_CAPACITY {
        hauler.amount_requested = HAULER_CAPACITY
    } else {
        hauler.amount_requested = amount_available;
    }
    haul.amount_reserved += hauler.amount_requested;
    haul.required_haulers -= 1;
    haul.hauler_list.remove(&entity);
}
