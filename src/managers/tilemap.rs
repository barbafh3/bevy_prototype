use bevy::prelude::{
    AssetServer, Assets, ColorMaterial, Commands, Handle, Res, ResMut, SpriteComponents, Texture,
    Transform,
};

pub struct TileMap {
    matrix: Vec<Vec<Tile>>,
    layer: i32,
}
pub struct Tile {
    sprite: SpriteComponents,
}

impl TileMap {
    pub fn new(
        height: i32,
        width: i32,
        layer: i32,
        commands: &Commands,
        asset_server: &Res<AssetServer>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> TileMap {
        let texture: Handle<Texture> = asset_server.load("assets/ralph_wolf.png").unwrap();
        let mut map_vector: Vec<Vec<Tile>> = Vec::with_capacity(height as usize);
        for i in 0..height {
            let vec: Vec<Tile> = Vec::with_capacity(width as usize);
            map_vector.insert(i as usize, vec)
        }
        for mut tile_vector in &mut map_vector {
            let length = tile_vector.capacity() - 1;
            for i in 0..length {
                // let local_texture = texture.clone();
                tile_vector.insert(
                    i as usize,
                    Tile {
                        sprite: SpriteComponents {
                            material: materials.add(texture.into()),
                            transform: Transform::from_scale(0.5),
                            ..Default::default()
                        },
                    },
                );
            }
        }
        return TileMap {
            matrix: map_vector,
            layer,
        };
    }
}
