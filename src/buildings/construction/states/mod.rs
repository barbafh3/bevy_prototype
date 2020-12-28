pub mod construction;
pub mod loading;
pub mod placing;

use self::{
    construction::state_construction_work, loading::state_loading_construction,
    placing::state_placing_construction,
};

use super::Construction;
use crate::{
    buildings::CurrentBuilding, camera::CameraData, managers::tasks::build::BuilderRequest,
};
use bevy::{
    ecs::{Commands, Entity, Query, Res, ResMut},
    input::Input,
    prelude::{Events, MouseButton, Transform},
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

pub fn sys_run_construction_states(
    commands: &mut Commands,
    mut events: ResMut<Events<BuilderRequest>>,
    camera_data: Res<CameraData>,
    mouse_input: Res<Input<MouseButton>>,
    mut current_building: ResMut<CurrentBuilding>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Construction,
        &Transform,
        &mut RigidBodyHandleComponent,
    )>,
) {
    for (entity, mut construction, transform, rb_handle) in query.iter_mut() {
        match construction.state {
            super::ConstructionStates::Placing => state_placing_construction(
                commands,
                &mouse_input,
                &mut current_building,
                &mut construction,
                &camera_data,
                &mut rb_set,
                rb_handle,
            ),
            super::ConstructionStates::Loading => {
                state_loading_construction(commands, &transform, &mut events, construction, entity)
            }
            super::ConstructionStates::Construction => {
                state_construction_work(commands, &entity, &mut construction)
            }
        }
    }
}
