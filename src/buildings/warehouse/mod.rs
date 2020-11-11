use bevy::{
    core::Time,
    ecs::{Commands, Query, Res, ResMut},
    input::Input,
    math::Vec3,
    prelude::AssetServer,
    prelude::MouseButton,
    prelude::{Assets, Handle, Transform},
    sprite::ColorMaterial,
};

use crate::camera::CameraData;

use self::states::{
    construction_state::state_warehouse_construction, idle_state::state_warehouse_idle,
    placing_state::state_placing_warehouse,
};

use super::CurrentBuilding;

pub mod states;

pub struct Warehouse {
    pub state: WarehouseStates,
    pub construction_time: f32,
    pub warehouse_sprite_added: bool,
}

impl Warehouse {
    pub fn new() -> Warehouse {
        Warehouse {
            state: WarehouseStates::Placing,
            construction_time: 10.0,
            warehouse_sprite_added: false,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum WarehouseStates {
    Placing,
    Construction,
    Idle,
}

pub fn run_warehouse_states(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_data: Res<CameraData>,
    mouse_input: Res<Input<MouseButton>>,
    mut current_building: ResMut<CurrentBuilding>,
    mut query: Query<(&mut Warehouse, &mut Transform, &mut Handle<ColorMaterial>)>,
) {
    if !current_building.entity.is_none() {
        if let Ok((warehouse, mut transform, _)) = query.get_mut(current_building.entity.unwrap()) {
            if warehouse.state == WarehouseStates::Placing {
                transform.translation = Vec3::new(
                    camera_data.position.x(),
                    camera_data.position.y(),
                    transform.translation.z(),
                );
            }
        }
    }
    for (mut warehouse, _, material) in query.iter_mut() {
        match warehouse.state {
            WarehouseStates::Placing => state_placing_warehouse(
                &mut commands,
                &mouse_input,
                &mut current_building,
                &mut warehouse,
            ),
            WarehouseStates::Construction => state_warehouse_construction(&time, warehouse),
            WarehouseStates::Idle => {
                state_warehouse_idle(&asset_server, &mut materials, warehouse, material)
            }
        }
    }
}
