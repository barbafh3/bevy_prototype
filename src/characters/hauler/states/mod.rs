use self::idle::state_hauler_idle;
use super::Hauler;
use bevy::{
    core::Time,
    ecs::{Query, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

pub mod idle;

pub enum HaulerStates {
    Idle,
    Loading,
    Carrying,
}

pub fn sys_run_hauler_state(
    time: Res<Time>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(&mut Hauler, &Transform, &mut RigidBodyHandleComponent)>,
) {
    for (hauler, transform, rb_handle) in query.iter_mut() {
        match hauler.state {
            HaulerStates::Idle => state_hauler_idle(
                time.delta_seconds,
                hauler,
                transform,
                &mut rb_set,
                rb_handle,
            ),
            _ => (),
        }
    }
}
