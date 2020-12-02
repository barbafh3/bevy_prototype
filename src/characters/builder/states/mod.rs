use bevy::{
    core::Time,
    ecs::{Commands, Entity, Query, QuerySet, Res, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::buildings::{stockpile::Stockpile, warehouse::Warehouse};

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
    transform_query: Query<&Transform>,
    mut query_set: QuerySet<(Query<&mut Warehouse>, Query<&mut Stockpile>)>,
) {
    for (entity, mut builder, transform, rb_handle) in query.iter_mut() {
        match builder.state {
            BuilderStates::Idle => state_builder_idle(),
            BuilderStates::Working => state_builder_working(&mut builder, &mut query_set),
        }
    }
}
