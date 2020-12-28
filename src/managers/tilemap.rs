use std::{error::Error, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TileMap {
    tiles: Vec<Vec<i32>>,
}

pub fn load_tilemap<P: AsRef<Path>>(path: P) -> Result<TileMap, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tilemap: TileMap = serde_json::from_reader(reader)?;
    println!("Tilemap: {:?}", tilemap.tiles);
    Ok(tilemap)
}
