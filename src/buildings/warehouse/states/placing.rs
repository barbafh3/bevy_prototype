use super::WarehouseStates;
use crate::{buildings::warehouse::Warehouse, buildings::CurrentBuilding, camera::CameraData};
use bevy::{
    ecs::Mut,
    ecs::ResMut,
    ecs::{Commands, Res},
    input::Input,
    prelude::MouseButton,
};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
    rapier::math::Isometry,
};

pub fn state_placing_warehouse(
    commands: &mut Commands,
    mouse_input: &Res<Input<MouseButton>>,
    current_building: &mut ResMut<CurrentBuilding>,
    warehouse: &mut Mut<Warehouse>,
    camera_data: &Res<CameraData>,
    rb_set: &mut ResMut<RigidBodySet>,
    rb_handle: Mut<RigidBodyHandleComponent>,
) {
    let rb_index = rb_handle.handle();
    let mut rb = rb_set.get_mut(rb_index).unwrap();
    rb.set_position(
        Isometry::new(
            Vector2::new(camera_data.position.x(), camera_data.position.y()),
            0.0,
        ),
        true,
    );
    if mouse_input.just_released(MouseButton::Right) && current_building.entity.is_some() {
        commands.despawn(current_building.entity.unwrap());
        current_building.entity = None;
    }
    if mouse_input.just_released(MouseButton::Left) {
        warehouse.state = WarehouseStates::Loading;
        current_building.entity = None;
    }
}
