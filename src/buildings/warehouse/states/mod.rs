use bevy::{
    core::Time, ecs::Commands, ecs::Query, ecs::Res, ecs::ResMut, input::Input,
    prelude::AssetServer, prelude::Assets, prelude::Handle, prelude::MouseButton,
    sprite::ColorMaterial,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{buildings::CurrentBuilding, camera::CameraData};

use self::{
    construction_state::state_warehouse_construction, idle_state::state_warehouse_idle,
    placing_state::state_placing_warehouse,
};

use super::Warehouse;

#[derive(Eq, PartialEq)]
pub enum WarehouseStates {
    Placing,
    Construction,
    Idle,
}

pub mod construction_state;
pub mod idle_state;
pub mod placing_state;

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
        &mut Warehouse,
        &mut Handle<ColorMaterial>,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (mut warehouse, material, rb_handle) in query.iter_mut() {
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
        }
    }
}
