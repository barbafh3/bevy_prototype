use crate::{
    buildings::construction::{Construction, ConstructionStates},
    constants::tasks::MAXIMUM_BUILDERS,
    managers::tasks::{build::BuilderRequest, haul::Haul, GameTask},
};
use bevy::{
    ecs::{Commands, Entity, Mut, ResMut},
    math::{Vec2, Vec3},
    prelude::{Events, Transform},
};

pub fn state_loading_construction(
    commands: &mut Commands,
    transform: &Transform,
    events: &mut ResMut<Events<BuilderRequest>>,
    mut construction: Mut<Construction>,
    entity: Entity,
) {
    if has_finished_loading(&construction) {
        println!("Warehouse: Finished loading materials");
        events.send(BuilderRequest {
            amount: MAXIMUM_BUILDERS,
            construction: entity,
            position: Vec3::new(transform.translation.x(), transform.translation.y(), 100.0),
        });
        construction.state = ConstructionStates::Construction;
    } else if !construction.has_requested_resources {
        create_haul_tasks(commands, &mut construction, &entity);
        construction.has_requested_resources = true;
    }
}

pub(crate) fn has_finished_loading(construction: &Mut<Construction>) -> bool {
    let mut finished_loading: bool = true;
    for (_, amount) in construction.required_resources.iter() {
        if amount.clone() > 0 {
            finished_loading = false;
        }
    }
    return finished_loading;
}

pub(crate) fn create_haul_tasks(
    commands: &mut Commands,
    construction: &mut Mut<Construction>,
    entity: &Entity,
) {
    for (resource, amount) in construction.required_resources.iter() {
        if *amount > 0 {
            let haul: Haul = Haul::new(
                // 1.0,
                // 1.0,
                resource.clone(),
                amount.clone(),
                *entity,
                None,
            );
            commands.spawn((GameTask, haul));
        }
    }
}
