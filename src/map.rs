const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 50;
const TILE_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Room, Tunnel
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub height: usize,
    pub width: usize
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles : vec![TileType::Wall; TILE_COUNT],
            width : MAP_WIDTH,
            height: MAP_HEIGHT
        }
    }

    // Represents array index from a given x,y position
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }
}