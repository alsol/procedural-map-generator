use super::{Map, Rectangle, TileType};
use std::cmp::{min, max};

pub fn apply_room_to_map(map: &mut Map, room : &Rectangle) {
    for y in room.lower.y +1 ..= room.upper.y {
        for x in room.lower.x + 1 ..= room.upper.x {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Room;
        }
    }
}

pub fn apply_horizontal_tunnel(map: &mut Map, x_start:i32, x_end:i32, y:i32) {
    for x in min(x_start,x_end) ..= max(x_start,x_end) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width * map.height && map.tiles[idx] != TileType::Room {
            map.tiles[idx] = TileType::Tunnel
        }
    }
}

pub fn apply_vertical_tunnel(map: &mut Map, y_start:i32, y_end:i32, x:i32) {
    for y in min(y_start,y_end) ..= max(y_start,y_end) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width * map.height && map.tiles[idx] != TileType::Room {
            map.tiles[idx as usize] = TileType::Tunnel
        }
    }
}