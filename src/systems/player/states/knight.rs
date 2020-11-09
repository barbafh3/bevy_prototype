use bevy::{
    ecs::{Mut, Res, ResMut},
    prelude::{AssetServer, Assets, SpriteComponents},
    sprite::ColorMaterial,
};

pub fn player_knight_state(
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut sprite_components: Mut<SpriteComponents>,
) {
    let texture_handle = asset_server.load("knight.png");
    sprite_components.material = materials.add(texture_handle.into());
}
