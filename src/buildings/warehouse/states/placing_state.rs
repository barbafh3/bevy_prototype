use bevy::{
    ecs::Mut,
    ecs::ResMut,
    ecs::{Commands, Res},
    input::Input,
    prelude::MouseButton,
};

use crate::{
    buildings::warehouse::Warehouse, buildings::warehouse::WarehouseStates,
    buildings::CurrentBuilding,
};

pub fn state_placing_warehouse(
    commands: &mut Commands,
    mouse_input: &Res<Input<MouseButton>>,
    current_building: &mut ResMut<CurrentBuilding>,
    warehouse: &mut Mut<Warehouse>,
) {
    // if !current_building.entity.is_none() {
    //     if let Ok((mut warehouse, mut transform)) = query.get_mut(current_building.entity.unwrap())
    //     {
    //         if warehouse.state == WarehouseStates::Placing {
    //             transform.translation = Vec3::new(
    //                 camera_data.position.x(),
    //                 camera_data.position.y(),
    //                 transform.translation.z(),
    //             );
    if mouse_input.just_released(MouseButton::Right) && current_building.entity.is_some() {
        commands.despawn(current_building.entity.unwrap());
        current_building.entity = None;
    }
    if mouse_input.just_released(MouseButton::Left) {
        warehouse.state = WarehouseStates::Construction;
        current_building.entity = None;
    }
    // }
    // }
    // }
}
