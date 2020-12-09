pub mod construction;
pub mod loading;
pub mod placing;

use self::{
    construction::state_construction_work, loading::state_loading_construction,
    placing::state_placing_construction,
};

use super::Construction;
use crate::{buildings::CurrentBuilding, camera::CameraData};
use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, Res, ResMut},
    input::Input,
    prelude::MouseButton,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

pub fn sys_run_construction_states(
    mut commands: Commands,
    time: Res<Time>,
    // asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    camera_data: Res<CameraData>,
    mouse_input: Res<Input<MouseButton>>,
    mut current_building: ResMut<CurrentBuilding>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(Entity, &mut Construction, &mut RigidBodyHandleComponent)>,
) {
    for (entity, mut construction, rb_handle) in query.iter_mut() {
        match construction.state {
            super::ConstructionStates::Placing => state_placing_construction(
                &mut commands,
                &mouse_input,
                &mut current_building,
                &mut construction,
                &camera_data,
                &mut rb_set,
                rb_handle,
            ),
            super::ConstructionStates::Loading => {
                state_loading_construction(&mut commands, construction, &entity)
            }
            super::ConstructionStates::Construction => {
                state_construction_work(&time, &mut commands, &entity, &mut construction)
            }
        }
    }
}
