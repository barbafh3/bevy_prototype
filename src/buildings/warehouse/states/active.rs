use crate::buildings::warehouse::Warehouse;
use bevy::{
    ecs::{Mut, Res, ResMut},
    prelude::{AssetServer, Assets, Handle},
    sprite::ColorMaterial,
};

pub fn state_warehouse_active(
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut warehouse: Mut<Warehouse>,
    mut material: Mut<Handle<ColorMaterial>>,
) {
    if !warehouse.is_sprite_set {
        let texture_handle = asset_server.load("warehouse.png");
        *material = materials.add(texture_handle.into());
        warehouse.is_sprite_set = true;
        println!("Warehouse is idle!");
    }
}
