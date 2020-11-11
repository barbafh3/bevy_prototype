use bevy::{
    core::Time,
    ecs::{Mut, Res},
};

use crate::buildings::warehouse::{Warehouse, WarehouseStates};

pub fn state_warehouse_construction(time: &Res<Time>, mut warehouse: Mut<Warehouse>) {
    if warehouse.state == WarehouseStates::Construction {
        warehouse.construction_time = run_construction_tick(&warehouse, time.delta_seconds);
        println!("Warehouse under construction...");
        if warehouse.construction_time <= 0.0 {
            warehouse.state = WarehouseStates::Idle;
        }
    }
}

fn run_construction_tick(warehouse: &Mut<Warehouse>, delta: f32) -> f32 {
    if warehouse.construction_time > 0.0 {
        return warehouse.construction_time - delta;
    } else {
        return warehouse.construction_time;
    }
}
