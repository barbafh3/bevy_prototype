pub mod finished;
pub mod idle;
pub mod working;

use self::{
    finished::state_builder_finished_work, idle::state_builder_idle, working::state_builder_working,
};
use super::Builder;
use crate::{buildings::construction::Construction, characters::VillagerMovement};
use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

#[derive(Debug, PartialEq)]
pub enum BuilderStates {
    Idle,
    Working,
    Finished,
}

pub fn sys_run_builder_states(
    commands: &mut Commands,
    time: Res<Time>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Builder,
        &mut VillagerMovement,
        &Transform,
        &mut RigidBodyHandleComponent,
    )>,
    mut construction_query: Query<&mut Construction>,
) {
    for (entity, mut builder, mut movement, transform, rb_handle) in query.iter_mut() {
        match builder.state {
            BuilderStates::Idle => state_builder_idle(
                time.delta_seconds(),
                commands,
                entity,
                &mut builder,
                &mut movement,
                &transform,
                &mut rb_set,
                rb_handle,
            ),
            BuilderStates::Working => state_builder_working(
                time.delta_seconds(),
                entity,
                commands,
                &mut builder,
                &mut movement,
                &transform,
                &mut rb_set,
                rb_handle,
                &mut construction_query,
            ),
            BuilderStates::Finished => state_builder_finished_work(
                commands,
                entity,
                &mut builder,
                &mut movement,
                transform,
                &mut rb_set,
                rb_handle,
            ),
        }
    }
}
