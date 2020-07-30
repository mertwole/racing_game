use rand::{RngCore, rngs::StdRng};

use crate::engine::road::Road;
use crate::game::Game;

#[readonly::make]
pub struct RoadPath {
    pub source_id : usize,
    pub destination_id : usize,

    road : Road
}

impl RoadPath {
    pub fn new(source_id : usize, destination_id : usize) -> RoadPath {
        RoadPath { source_id, destination_id, road : Road::new(Game::load_image_rgb("road_tex.png")) }
    }

    pub fn generate(&mut self, rng : &mut StdRng, difficulty : f32) {

    }

    pub fn get_road<'a>(&'a self) -> &'a Road{
        &self.road
    }
}