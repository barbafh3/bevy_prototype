use std::collections::HashMap;

use crate::{
    buildings::warehouse::Warehouse,
    managers::tasks::{haul::Haul, GameTask},
};
use bevy::ecs::{Commands, Entity, Mut};

pub fn state_warehouse_loading(
    commands: &mut Commands,
    mut warehouse: Mut<Warehouse>,
    entity: &Entity,
) {
    if has_finished_loading(&warehouse) {
        warehouse.state = super::WarehouseStates::Construction;
    } else {
        create_haul_tasks(commands, &mut warehouse, entity);
    }
}

pub(crate) fn has_finished_loading(warehouse: &Mut<Warehouse>) -> bool {
    let mut finished_loading: bool = false;
    for (_, amount) in warehouse.required_resources.iter() {
        if *amount <= 0 {
            finished_loading = true;
        } else {
            finished_loading = false
        }
    }
    return finished_loading;
}

pub(crate) fn create_haul_tasks(
    commands: &mut Commands,
    warehouse: &mut Mut<Warehouse>,
    entity: &Entity,
) {
    for (resource, amount) in warehouse.required_resources.iter() {
        if *amount > 0 {
            let haul: Haul = Haul::new(
                // 1.0,
                // 1.0,
                resource.clone(),
                amount.clone(),
                *entity,
                None,
                HashMap::new(),
            );
            commands.spawn((GameTask, haul));
        }
    }
}
