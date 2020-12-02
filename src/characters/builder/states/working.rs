use bevy::ecs::{Mut, Query, QuerySet};

use crate::{
    buildings::stockpile::Stockpile, buildings::warehouse::Warehouse, characters::builder::Builder,
};

enum PossibleBuildings {
    Warehouse,
    Stockpile,
    WoodcuttersHut,
    None,
}

pub fn state_builder_working(
    builder: &mut Mut<Builder>,
    query_set: &mut QuerySet<(Query<&mut Warehouse>, Query<&mut Stockpile>)>,
) {
    let requested_and_current_building_exist =
        !builder.requested_construction.is_none() && !builder.current_construction.is_none();
    let is_inside_requested_construction = builder.is_inside_building
        && builder.requested_construction.unwrap() == builder.current_construction.unwrap();
    if requested_and_current_building_exist && is_inside_requested_construction {}
}
