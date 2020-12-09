use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, QuerySet, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::buildings::{construction::Construction, stockpile::Stockpile, warehouse::Warehouse};

use self::{idle::state_builder_idle, working::state_builder_working};

use super::Builder;

pub mod idle;
pub mod working;

#[derive(Debug, PartialEq)]
pub enum BuilderStates {
    Idle,
    Working,
}

pub fn sys_run_builder_states(
    mut commands: Commands,
    time: Res<Time>,
    mut rb_set: ResMut<RigidBodySet>,
    mut query: Query<(
        Entity,
        &mut Builder,
        &Transform,
        &mut RigidBodyHandleComponent,
    )>,
    mut construction_query: Query<&mut Construction>,
) {
    for (entity, mut builder, transform, rb_handle) in query.iter_mut() {
        match builder.state {
            BuilderStates::Idle => state_builder_idle(
                time.delta_seconds,
                &mut commands,
                entity,
                &mut builder,
                &transform,
                &mut rb_set,
                rb_handle,
            ),
            BuilderStates::Working => state_builder_working(
                entity,
                &mut commands,
                &mut builder,
                &transform,
                &mut rb_set,
                rb_handle,
                &mut construction_query,
            ),
        }
    }
}
