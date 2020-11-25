use crate::{
    buildings::warehouse::Warehouse,
    managers::tasks::{haul::Haul, TASK_MANAGER},
};
use bevy::ecs::{Entity, Mut};

pub fn state_warehouse_loading(mut warehouse: Mut<Warehouse>, entity: &Entity) {
    // println!("Warehouse loading!");
    if has_finished_loading(&warehouse) {
        warehouse.state = super::WarehouseStates::Construction;
    } else {
        // create_haul_tasks(&mut warehouse, entity);
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

pub(crate) fn create_haul_tasks(warehouse: &mut Mut<Warehouse>, entity: &Entity) {
    let task_manager = &mut TASK_MANAGER.lock().unwrap();
    for (resource, amount) in warehouse.required_resources.iter() {
        if *amount > 0 {
            let haul = Haul::new(1.0, 1.0, resource.clone(), amount.clone(), *entity, None);
            task_manager.register_task(haul);
        }
    }
}
