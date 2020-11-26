use super::WarehouseStates;
use crate::{buildings::warehouse::Warehouse, camera::CameraData};
use bevy::{
    core::Time,
    ecs::{Mut, Res, ResMut},
};
use bevy_rapier2d::{
    na::Vector2,
    physics::RigidBodyHandleComponent,
    rapier::{dynamics::RigidBodySet, math::Isometry},
};

pub fn state_warehouse_construction(
    time: &Res<Time>,
    mut warehouse: Mut<Warehouse>,
    camera_data: &Res<CameraData>,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    if !warehouse.is_position_set {
        let rb_index = rb_handle.handle();
        let mut rb = rb_set.get_mut(rb_index).unwrap();
        rb.position = Isometry::new(
            Vector2::new(camera_data.position.x(), camera_data.position.y()),
            0.0,
        );
        warehouse.is_position_set = true;
    }
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
