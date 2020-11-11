use bevy::{
    ecs::{Mut, Res, ResMut},
    prelude::{AssetServer, Assets, Handle},
    sprite::ColorMaterial,
};

use crate::buildings::warehouse::{Warehouse, WarehouseStates};

pub fn state_warehouse_idle(
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut warehouse: Mut<Warehouse>,
    mut material: Mut<Handle<ColorMaterial>>,
) {
    if warehouse.state == WarehouseStates::Idle {
        if !warehouse.warehouse_sprite_added {
            let texture_handle = asset_server.load("warehouse.png");
            *material = materials.add(texture_handle.into());
            warehouse.warehouse_sprite_added = true;
        }
        println!("Warehouse is idle!");
    }
}
