pub mod construction_state;
pub mod idle_state;
pub mod loading_state;
pub mod placing_state;

use self::{
    construction_state::state_warehouse_construction, idle_state::state_warehouse_idle,
    loading_state::state_warehouse_loading, placing_state::state_placing_warehouse,
};
use super::Warehouse;
use crate::{buildings::CurrentBuilding, camera::CameraData, managers::tasks::TaskManager};
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
    Placing,
    Loading,
    Construction,
    Idle,
}

pub fn sys_run_warehouse_states(
    mut commands: Commands,
    time: Res<Time>,
    mut task_manager: ResMut<TaskManager>,
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
            WarehouseStates::Placing => state_placing_warehouse(
                &mut commands,
                &mouse_input,
                &mut current_building,
                &mut warehouse,
                &camera_data,
                &mut rb_set,
                rb_handle,
            ),
            WarehouseStates::Construction => {
                state_warehouse_construction(&time, warehouse, &camera_data, &mut rb_set, rb_handle)
            }
            WarehouseStates::Idle => {
                state_warehouse_idle(&asset_server, &mut materials, warehouse, material)
            }
            WarehouseStates::Loading => {
                state_warehouse_loading(&mut task_manager, warehouse, &entity)
            }
        }
    }
}
