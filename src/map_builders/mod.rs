use self::bsp_map::BspMapBuilder;

use super::{Map, Rectangle, TileType};
use simple_map::SimpleMapBuilder;
mod common;
use common::*;

pub mod simple_map;
pub mod bsp_map;

pub trait MapBuilder {
    fn name(&self) -> &str;
    fn build(&mut self);
    fn get_map(&mut self) -> Map;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn simple_builder() -> Box<dyn MapBuilder> {
    Box::new(SimpleMapBuilder::new())
}

pub fn bsp_builder() -> Box<dyn MapBuilder> {
    Box::new(BspMapBuilder::new())
}

pub fn random_builder() -> Box<dyn MapBuilder> {
    if rand::random() {
        bsp_builder()
    } else {
        simple_builder()
    }
}