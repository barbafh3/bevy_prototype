pub mod carrying;
pub mod idle;
pub mod loading;

use self::{
    carrying::state_hauler_carrying, idle::state_hauler_idle, loading::state_hauler_loading,
};
use super::Hauler;
use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum HaulerStates {
    Idle,
    Loading,
    Carrying,
}

pub fn sys_run_hauler_state(
    mut commands: Commands,
    time: Res<Time>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Hauler,
        &Transform,
        &mut RigidBodyHandleComponent,
    )>,
    transform_query: Query<&Transform>,
) {
    for (entity, mut hauler, transform, rb_handle) in query.iter_mut() {
        match hauler.state {
            HaulerStates::Idle => state_hauler_idle(
                time.delta_seconds,
                &mut commands,
                entity,
                &mut hauler,
                transform,
                &mut rb_set,
                rb_handle,
            ),
            HaulerStates::Loading => {
                state_hauler_loading(
                    entity,
                    &mut commands,
                    &mut hauler,
                    transform,
                    &mut rb_set,
                    rb_handle,
                    &transform_query,
                );
            }
            HaulerStates::Carrying => state_hauler_carrying(
                &mut hauler,
                transform,
                &mut rb_set,
                rb_handle,
                &transform_query,
            ),
        }
    }
}
