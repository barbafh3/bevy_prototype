pub mod active;
pub mod inactive;

use self::active::state_warehouse_active;
use super::Warehouse;
use bevy::{
    ecs::Query,
    ecs::Res,
    ecs::{Entity, ResMut},
    prelude::AssetServer,
    prelude::Assets,
    prelude::Handle,
    sprite::ColorMaterial,
};
use bevy_rapier2d::physics::RigidBodyHandleComponent;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum WarehouseStates {
    Active,
    // Inactive,
}

pub fn sys_run_warehouse_states(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        Entity,
        &mut Warehouse,
        &mut Handle<ColorMaterial>,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (_, warehouse, material, _) in query.iter_mut() {
        match warehouse.state {
            WarehouseStates::Active => {
                state_warehouse_active(&asset_server, &mut materials, warehouse, material)
            } // WarehouseStates::Inactive => state_warehouse_inactive(),
        }
    }
}
