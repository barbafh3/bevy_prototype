pub mod active;
pub mod inactive;

use self::{active::state_warehouse_active, inactive::state_warehouse_inactive};

use super::Warehouse;
use crate::{buildings::CurrentBuilding, camera::CameraData};
use bevy::{
    core::Time,
    ecs::Commands,
    ecs::Query,
    ecs::Res,
    ecs::{Entity, ResMut},
    input::Input,
    prelude::AssetServer,
    prelude::Assets,
    prelude::Handle,
    prelude::MouseButton,
    sprite::ColorMaterial,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum WarehouseStates {
    Active,
    Inactive,
}

pub fn sys_run_warehouse_states(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_data: Res<CameraData>,
    mouse_input: Res<Input<MouseButton>>,
    mut current_building: ResMut<CurrentBuilding>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Warehouse,
        &mut Handle<ColorMaterial>,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (entity, mut warehouse, material, rb_handle) in query.iter_mut() {
        match warehouse.state {
            WarehouseStates::Active => {
                state_warehouse_active(&asset_server, &mut materials, warehouse, material)
            }
            WarehouseStates::Inactive => state_warehouse_inactive(),
        }
    }
}
