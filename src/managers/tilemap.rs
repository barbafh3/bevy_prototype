// use bevy::prelude::Vec2;

pub struct TileMap {
    pub id: i32,
    // pub matrix: Vec<Vec<Tile>>,
    pub layer: i32,
}

// pub struct Tile {
//     pub tilemap_id: i32,
//     pub position: Vec2,
// }

impl TileMap {
    pub fn new(id: i32, layer: i32) -> TileMap {
        // let mut map_vector: Vec<Vec<Tile>> = Vec::with_capacity(height as usize);
        // for i in 0..height {
        //     let vec: Vec<Tile> = Vec::with_capacity(width as usize);
        //     map_vector.insert(i as usize, vec)
        // }
        return TileMap {
            id,
            // matrix: map_vector,
            layer,
        };
    }
}
