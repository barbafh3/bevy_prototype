pub mod carrying;
pub mod finished;
pub mod idle;
pub mod loading;

use crate::characters::VillagerMovement;

use self::{
    carrying::state_hauler_carrying, finished::state_hauler_finished_work, idle::state_hauler_idle,
    loading::state_hauler_loading,
};
use super::Hauler;
use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum HaulerStates {
    Idle,
    Loading,
    Carrying,
    Finished,
}

pub fn sys_run_hauler_state(
    commands: &mut Commands,
    time: Res<Time>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Hauler,
        &mut VillagerMovement,
        &Transform,
        &mut RigidBodyHandleComponent,
    )>,
    transform_query: Query<&Transform>,
) {
    for (entity, mut hauler, mut movement, transform, rb_handle) in query.iter_mut() {
        match hauler.state {
            HaulerStates::Idle => state_hauler_idle(
                time.delta_seconds(),
                commands,
                entity,
                &mut hauler,
                &mut movement,
                transform,
                &mut rb_set,
                rb_handle,
            ),
            HaulerStates::Loading => {
                state_hauler_loading(
                    commands,
                    entity,
                    &mut hauler,
                    &mut movement,
                    transform,
                    &mut rb_set,
                    rb_handle,
                    &transform_query,
                );
            }
            HaulerStates::Carrying => state_hauler_carrying(
                &mut hauler,
                &mut movement,
                transform,
                &mut rb_set,
                rb_handle,
                &transform_query,
            ),
            HaulerStates::Finished => state_hauler_finished_work(
                commands,
                entity,
                &mut hauler,
                &mut movement,
                transform,
                &mut rb_set,
                rb_handle,
            ),
        }
    }
}
