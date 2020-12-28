use crate::{
    buildings::construction::Construction,
    characters::{builder::Builder, VillagerMovement},
    get_idle_point,
    managers::villagers::IdleVillager,
};
use bevy::{
    ecs::{Commands, Entity, Mut, Query, ResMut},
    prelude::Transform,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use super::BuilderStates;

// enum PossibleBuildings {
//     Warehouse,
//     Stockpile,
//     WoodcuttersHut,
//     None,
// }

pub fn state_builder_working(
    delta: f32,
    entity: Entity,
    commands: &mut Commands,
    builder: &mut Mut<Builder>,
    movement: &mut Mut<VillagerMovement>,
    transform: &Transform,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
    construction_query: &mut Query<&mut Construction>,
) {
    let rb_index = rb_handle.handle();
    let rb = rb_set.get_mut(rb_index).unwrap();

    if builder.is_idle {
        commands.remove_one::<IdleVillager>(entity);
        builder.is_idle = false;
    }
    if let Ok(mut construction) =
        construction_query.get_mut(builder.requested_construction.unwrap())
    {
        let vector = movement.target - transform.translation;
        let is_far_enough = vector.x.abs() > 2.0 && vector.y.abs() > 2.0;
        if is_far_enough {
            let target_vector = Vector2::new(vector.x, vector.y);
            let direction = target_vector.normalize();
            rb.set_linvel(direction * movement.speed, true);
        } else {
            let requested_and_current_building_exist = !builder.requested_construction.is_none()
                && !builder.current_construction.is_none();
            let is_inside_requested_construction = builder.is_inside_building
                && builder.requested_construction.unwrap() == builder.current_construction.unwrap();

            if requested_and_current_building_exist {
                rb.set_linvel(Vector2::new(0.0, 0.0), true);
                if is_inside_requested_construction {
                    if construction.construction_time > 0.0 {
                        construction.construction_time -= builder.construction_tick * delta;
                    } else {
                        println!("BuilderWorking: Builder is now idle");
                        movement.target = get_idle_point();
                        builder.requested_construction = None;
                        builder.state = BuilderStates::Finished;
                    }
                }
            }
        }
    }
}
