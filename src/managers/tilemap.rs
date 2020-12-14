use bevy::{
    asset::LoadState, prelude::*, render::texture::TextureFormat, sprite::TextureAtlasBuilder,
};
use bevy_tilemap::{
    chunk::{Chunk, WorldChunk},
    dimensions::Dimensions2,
    map::{TileMap, WorldMap},
    tile::{Tile, TileSetter},
};

#[derive(Debug, Default, Clone)]
pub struct WorldTile {
    pub texture: Handle<Texture>,
    pub coord: Vec2,
}

impl Tile for WorldTile {
    const WIDTH: f32 = 16.0;
    const HEIGHT: f32 = 16.0;

    fn texture(&self) -> Option<&Handle<Texture>> {
        Some(&self.texture)
    }

    fn coord(&self) -> Option<Vec2> {
        Some(self.coord)
    }
}

#[derive(Default, Clone)]
pub struct TileSpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub atlas_loaded: bool,
}

#[derive(Default, Clone)]
pub struct MapState {
    pub map_loaded: bool,
}

pub fn load_atlas(
    mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);
        map.set_texture_atlas(atlas_handle);
        sprite_handles.atlas_loaded = true;
    }
}

pub fn build_tilemap(
    mut map: ResMut<WorldMap<WorldTile, WorldChunk<WorldTile>>>,
    mut map_state: ResMut<MapState>,
    mut chunks: ResMut<Assets<WorldChunk<WorldTile>>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if map_state.map_loaded {
        return;
    }

    let pixel_width = map.dimensions().x() * WorldChunk::<WorldTile>::WIDTH;
    let pixel_height = map.dimensions().y() * WorldChunk::<WorldTile>::HEIGHT;

    println!("Tilemap dimensions - x: {} y:{}", pixel_width, pixel_height);

    for x in 0..map.dimensions().x() as i32 {
        for y in 0..map.dimensions().y() as i32 {
            let coordinates = Vec2::new(x as f32, y as f32);
            let mut chunk = WorldChunk::default();
            let texture = Texture::new_fill(
                chunk.pixel_dimensions(),
                &[255, 0, 0, 255],
                TextureFormat::Rgba8UnormSrgb,
            );
            let texture_handle = textures.add(texture);
            chunk.set_texture_handle(Some(texture_handle));
            map.add_chunk(chunk, coordinates, &mut chunks)
        }
    }

    let mut floor_sprite = asset_server.get_handle("textures/grass_tall_light.png");
    floor_sprite.make_strong(&mut textures);
    let floor_tile = WorldTile {
        texture: floor_sprite,
        coord: Vec2::new(0.0, 0.0),
    };

    let mut setter = TileSetter::with_capacity((pixel_height * pixel_width) as usize);
    for x in 0..pixel_width as i32 {
        for y in 0..pixel_height as i32 {
            setter.push(Vec3::new(x as f32, y as f32, 0.0), floor_tile.clone());
        }
    }

    map.set_tiles(setter);
    map_state.map_loaded = true;
}
