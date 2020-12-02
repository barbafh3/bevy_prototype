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
        println!("Warehouse: Finished loading materials");
        warehouse.state = super::WarehouseStates::Construction;
    } else if !warehouse.has_requested_resources {
        create_haul_tasks(commands, &mut warehouse, entity);
        warehouse.has_requested_resources = true;
    }
}

pub(crate) fn has_finished_loading(warehouse: &Mut<Warehouse>) -> bool {
    let mut finished_loading: bool = true;
    for (_, amount) in warehouse.required_resources.iter() {
        if amount.clone() > 0 {
            finished_loading = false;
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
            );
            commands.spawn((GameTask, haul));
        }
    }
}
