pub mod active;
pub mod disabled;

use self::{active::state_warehouse_active, disabled::state_warehouse_disabled};
use super::Warehouse;
use crate::buildings::DisabledBuilding;
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

pub fn sys_run_warehouse_states(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        Entity,
        &mut Warehouse,
        Option<&DisabledBuilding>,
        &mut Handle<ColorMaterial>,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (_, warehouse, disabled_building, material, _) in query.iter_mut() {
        match disabled_building {
            None => state_warehouse_active(&asset_server, &mut materials, warehouse, material),
            Some(_) => state_warehouse_disabled(),
        }
    }
}
