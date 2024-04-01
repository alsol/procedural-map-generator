use super::{Map, Rectangle, apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel};
use super::MapBuilder;
use rand::prelude::*;

pub struct SimpleMapBuilder {
    map: Map,
    history: Vec<Map>
}

impl MapBuilder for SimpleMapBuilder {
    fn name(&self) -> &str {
        "Simple"
    }

    fn build(&mut self) {
        SimpleMapBuilder::rooms_and_corridors(self);
    }
    
    fn get_map(&mut self) -> Map {
        self.map.clone()
    }
    
    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }
    
    fn take_snapshot(&mut self) {
        let snapshot = self.map.clone();
        self.history.push(snapshot)
    }
}

impl SimpleMapBuilder {
    pub fn new() -> SimpleMapBuilder {
        SimpleMapBuilder {
            map: Map::new(),
            history: Vec::new()
        }
    }

    fn rooms_and_corridors(&mut self) {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = rand::thread_rng();

        let mut rooms: Vec<Rectangle> = Vec::new();

        self.take_snapshot();

        for _ in 0..MAX_ROOMS {

            let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);

            let x = rng.gen_range(1..(self.map.width as i32 - w - 1)) - 1;
            let y = rng.gen_range(1..(self.map.height as i32 - h - 1)) - 1;

            let new_room = Rectangle::create(x, y, w, h);

            let ok = rooms.iter().all(|room| !room.intersect(&new_room));
        
            if ok {
                apply_room_to_map(&mut self.map, &new_room);      
                self.take_snapshot();

                if !rooms.is_empty() {
                    let new = new_room.center();
                    let prev = rooms[rooms.len()-1].center();

                    if rng.gen() {
                        apply_horizontal_tunnel(&mut self.map, prev.x, new.x, prev.y);
                        apply_vertical_tunnel(&mut self.map, prev.y, new.y, new.x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev.y, new.y, prev.x);
                        apply_horizontal_tunnel(&mut self.map, prev.x, new.x, new.y);
                    }   
                }

                rooms.push(new_room);           
                self.take_snapshot(); 
            }
        }
    }
}
