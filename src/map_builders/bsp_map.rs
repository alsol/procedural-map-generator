use rand::{rngs::ThreadRng, Rng};

use crate::{map::TileType, Point};

use super::{Map, MapBuilder, Rectangle, apply_room_to_map, apply_vertical_tunnel, apply_horizontal_tunnel};

pub struct BspMapBuilder {
    map: Map,
    history: Vec<Map>,
    rooms: Vec<Rectangle>,
    rects: Vec<Rectangle>
}

impl MapBuilder for BspMapBuilder {
    fn name(&self) -> &str {
        "Binary Space Partition"
    }

    fn build(&mut self) {
        self.take_snapshot();

        self.rooms.clear();
        self.rects.clear();
        // Start with a single map-sized rectangle
        let first_room = Rectangle::create(2, 2, (self.map.width - 5) as i32, (self.map.height - 5) as i32);
        self.rects.push(first_room); 

        let mut rng = rand::thread_rng();

        self.add_subrects(first_room); // Divide the first room

        // Up to 240 times, we get a random rectangle and divide it. 
        // If its possible to squeeze a room in there, 
        // we place it and add it to the rooms list.
        for _ in 1..240 {
            let rect = self.get_random_rect(&mut rng);
            let candidate = self.get_random_sub_rect(rect, &mut rng);

            if self.is_possible(candidate) {
                self.rooms.push(candidate);
                self.add_subrects(rect);
                apply_room_to_map(&mut self.map, &candidate);
                self.take_snapshot();
            }
        }

        self.rooms.sort_by(|l, r| l.lower.x.cmp(&r.lower.x));

        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let next_room = self.rooms[i+1];
            let start_x = room.lower.x + rng.gen_range(0..room.width());
            let start_y = room.lower.y + (rng.gen_range(0..i32::abs(room.lower.y - room.upper.y)));
            let end_x = next_room.lower.x + (rng.gen_range(0..i32::abs(next_room.lower.x - next_room.upper.x)));
            let end_y = next_room.lower.y + (rng.gen_range(0..i32::abs(next_room.lower.y - next_room.upper.y)));
            if rng.gen() {
                apply_horizontal_tunnel(&mut self.map, start_x, end_x, start_y);
                apply_vertical_tunnel(&mut self.map, start_y, end_y, end_x);
            } else {
                apply_vertical_tunnel(&mut self.map, start_y, end_y, start_x);
                apply_horizontal_tunnel(&mut self.map, start_x, end_x, end_y);
            }   
            self.take_snapshot();
        }
        
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn take_snapshot(&mut self) {
       let map = self.map.clone();
       self.history.push(map)
    }
}

impl BspMapBuilder {

    // ###############        ###############
    // #             #        #  1   +   2  #
    // #             #        #      +      #
    // #      0      #   ->   #+++++++++++++#
    // #             #        #   3  +   4  #
    // #             #        #      +      #
    // ###############        ###############
    fn add_subrects(&mut self, rect : Rectangle) {
        let width = i32::abs(rect.lower.x - rect.upper.x);
        let height = i32::abs(rect.lower.y - rect.upper.y);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        let root = rect.lower;
    
        self.rects.push(Rectangle::create( root.x, root.y, half_width, half_height ));
        self.rects.push(Rectangle::create( root.x, root.y + half_height, half_width, half_height ));
        self.rects.push(Rectangle::create( root.x + half_width, root.y, half_width, half_height ));
        self.rects.push(Rectangle::create( root.x + half_width, root.y + half_height, half_width, half_height ));
    }

    // Get random rectangle from list
    fn get_random_rect(&mut self, rng : &mut ThreadRng) -> Rectangle {
        if self.rects.len() == 1 { return self.rects[0]; }
        let idx = rng.gen_range(1..self.rects.len()) as usize;
        self.rects[idx]
    }

    // ###############        ########
    // #             #        #   1  #
    // #             #        #      #
    // #      0      #   ->   ########
    // #             #
    // #             #
    // ###############
    fn get_random_sub_rect(&self, rect: Rectangle, rng : &mut ThreadRng) -> Rectangle {
        let mut result = rect;
        let rect_width = rect.width();
        let rect_height = rect.height();
    
        let w = i32::max(3, rng.gen_range(1..=i32::min(rect_width, 10))-1) + 1;
        let h = i32::max(3, rng.gen_range(1..=i32::min(rect_height, 10))-1) + 1;
    
        result.lower.x += rng.gen_range(1..=6)-1;
        result.lower.y += rng.gen_range(1..=6)-1;
        result.upper.x = result.lower.x + w;
        result.upper.y = result.lower.y + h;
    
        result
    }

    fn is_possible(&self, rect : Rectangle) -> bool {
        let mut expanded = rect;
        expanded.lower.x -= 2;
        expanded.lower.y += 2;
        expanded.upper.x -= 2;
        expanded.upper.y += 2;
    
        let mut can_build = true;
    
        for y in expanded.lower.y ..= expanded.upper.y {
            for x in expanded.lower.x ..= expanded.upper.x {
                if x > (self.map.width as i32)-2 { can_build = false; }
                if y > (self.map.height as i32)-2 { can_build = false; }
                if x < 1 { can_build = false; }
                if y < 1 { can_build = false; }
                if can_build {
                    let idx = self.map.xy_idx(x, y);
                    if self.map.tiles[idx] != TileType::Wall { 
                        can_build = false; 
                    }
                }
            }
        }
    
        can_build
    }

    fn draw_corridor(&mut self, start: Point, finish: Point) {
        let mut x = start.x;
        let mut y = start.x;
    
        while x != finish.x || y != finish.y {
            if x < finish.x {
                x += 1;
            } else if x > finish.x {
                x -= 1;
            } else if y < finish.y {
                y += 1;
            } else if y > finish.y {
                y -= 1;
            }
    
            let idx = self.map.xy_idx(x, y);
            self.map.tiles[idx] = TileType::Tunnel;
        }
    }

    pub fn new() -> BspMapBuilder {
        BspMapBuilder {
            map: Map::new(),
            history: Vec::new(),
            rooms: Vec::new(),
            rects: Vec::new()
        }
    }
}