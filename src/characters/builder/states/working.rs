use crate::{
    buildings::stockpile::Stockpile,
    buildings::{construction::Construction, warehouse::Warehouse},
    characters::{builder::Builder, normalize},
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, Query, QuerySet, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use super::BuilderStates;

enum PossibleBuildings {
    Warehouse,
    Stockpile,
    WoodcuttersHut,
    None,
}

pub fn state_builder_working(
    entity: Entity,
    commands: &mut Commands,
    builder: &mut Mut<Builder>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    construction_query: &mut Query<&mut Construction>,
) {
    println!("Builder state is 'Working'...");
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();

    if builder.is_idle {
        commands.remove_one::<IdleVillager>(entity);
        builder.is_idle = false;
    }
    if let Ok(mut construction) =
        construction_query.get_mut(builder.requested_construction.unwrap())
    {
        let requested_and_current_building_exist =
            !builder.requested_construction.is_none() && !builder.current_construction.is_none();
        let is_inside_requested_construction = builder.is_inside_building
            && builder.requested_construction.unwrap() == builder.current_construction.unwrap();

        if requested_and_current_building_exist {
            if is_inside_requested_construction {
                if construction.construction_time > 0.0 {
                    construction.construction_time -= builder.construction_tick;
                } else {
                    builder.state = BuilderStates::Idle;
                }
            } else {
                println!("Builder moving to {}", builder.movement_target.clone());
                let target_vector = builder.movement_target - transform.translation;
                let is_far_enough = target_vector.x().abs() > 2.0 && target_vector.y().abs() > 2.0;
                if is_far_enough {
                    let direction = normalize(target_vector);
                    rb.set_linvel(direction * builder.movement.speed, true);
                }
            }
        }
    }
}
