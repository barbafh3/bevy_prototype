use super::WarehouseStates;
use crate::buildings::warehouse::Warehouse;
use bevy::{
    core::Time,
    ecs::{Mut, Res},
};

pub fn state_warehouse_construction(time: &Res<Time>, mut warehouse: Mut<Warehouse>) {
    warehouse.construction_time = run_construction_tick(&warehouse, time.delta_seconds);
    if warehouse.construction_time <= 0.0 {
        warehouse.state = WarehouseStates::Idle;
    }
}

fn run_construction_tick(warehouse: &Mut<Warehouse>, delta: f32) -> f32 {
    if warehouse.construction_time > 0.0 {
        return warehouse.construction_time - delta;
    } else {
        return warehouse.construction_time;
    }
}
