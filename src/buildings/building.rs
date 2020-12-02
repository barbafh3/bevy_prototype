use bevy::ecs::Mut;

use super::warehouse::Warehouse;

pub trait ConstructionWork {
    fn do_construction_work(&mut self);
}

// pub enum PossibleConstructions {
//     Warehouse(Mut<Warehouse>),
// }
