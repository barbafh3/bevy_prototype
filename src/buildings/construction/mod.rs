pub mod states;

use crate::{
    characters::hauler::{states::HaulerStates, Hauler},
    constants::enums::GameResources,
};
use bevy::ecs::Mut;
use bevy_rapier2d::rapier::geometry::Proximity;
use enum_map::EnumMap;

pub enum ConstructionTypes {
    WarehouseParams(i32),
    // SmallHouse(SmallHouse),
    // WoodcuttersHutParams(i32, i32),
    // Sawmill(Sawmill),
}

pub enum ConstructionStates {
    Placing,
    Loading,
    Construction,
}

pub struct Construction {
    pub state: ConstructionStates,
    pub construction_component: ConstructionTypes,
    pub required_resources: EnumMap<GameResources, i32>,
    pub construction_time: f32,
    pub is_position_set: bool,
    pub has_requested_resources: bool,
}

impl Construction {
    pub fn new(
        construction_component: ConstructionTypes,
        required_resources: EnumMap<GameResources, i32>,
        construction_time: f32,
    ) -> Construction {
        Construction {
            state: ConstructionStates::Placing,
            construction_component,
            required_resources,
            construction_time,
            is_position_set: false,
            has_requested_resources: false,
        }
    }
}

pub fn on_construction_proximity_event(
    construction: &mut Mut<Construction>,
    event: Proximity,
    hauler: &mut Mut<Hauler>,
) {
    match event {
        Proximity::Intersecting => on_construction_intersect(construction, hauler),
        _ => (),
    }
}

fn on_construction_intersect(construction: &mut Mut<Construction>, hauler: &mut Mut<Hauler>) {
    match construction.state {
        ConstructionStates::Loading => {
            if hauler_is_carrying_resources(hauler) {
                take_resources(construction, hauler);
            }
        }
        _ => (),
    }
}

fn hauler_is_carrying_resources(hauler: &mut Mut<Hauler>) -> bool {
    match hauler.state {
        HaulerStates::Carrying => true,
        _ => false,
    }
}

fn take_resources(construction: &mut Mut<Construction>, hauler: &mut Mut<Hauler>) {
    if let Some(resource) = hauler.current_resource {
        if construction.required_resources[resource] >= hauler.capacity {
            construction.required_resources[resource] -= hauler.capacity;
            hauler.capacity = 0;
            hauler.amount_requested = 0;
        }
    }
}
