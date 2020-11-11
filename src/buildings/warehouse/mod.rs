pub mod states;

use bevy::{
    // core::Time,
    ecs::Query,
    // ecs::ResMut,
    ecs::Res,
    // input::Input,
    // prelude::AssetServer,
    // prelude::Assets,
    prelude::Transform,
    // prelude::{KeyCode, MouseButton},
    // sprite::ColorMaterial,
};

use crate::camera::CameraData;

use self::states::{placed_state::state_warehouse_placed, placing_state::state_placing_warehouse};

use super::Warehouse;

pub enum WarehouseStates {
    Placing,
    Placed,
}

pub fn sys_run_warehouse_state(
    // mut commands: Commands,
    // time: Res<Time>,
    // keyboard_input: Res<Input<KeyCode>>,
    // mouse_input: Res<Input<MouseButton>>,
    // asset_server: Res<AssetServer>,
    camera_data: Res<CameraData>,
    // mut current_building: ResMut<CurrentBuilding>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Warehouse, &mut Transform)>,
) {
    if query.iter_mut().len() > 0 {
        for (warehouse, transform) in query.iter_mut() {
            match warehouse.state {
                WarehouseStates::Placing => state_placing_warehouse(&camera_data, transform),
                WarehouseStates::Placed => state_warehouse_placed(),
            }
        }
    }
}
