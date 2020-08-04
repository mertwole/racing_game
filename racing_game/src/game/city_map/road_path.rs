use rand::{RngCore, rngs::StdRng};

use crate::engine::road::road_data::*;

#[readonly::make]
pub struct RoadPath {
    pub source_id : usize,
    pub destination_id : usize,

    meta : Option<RoadPathMeta>
}

#[derive(Clone)]
pub struct RoadPathMeta{
    pub length : f32,
    pub roads_data : Vec<RoadData>
}

impl RoadPath {
    pub fn new(source_id : usize, destination_id : usize) -> RoadPath {
        RoadPath { source_id, destination_id, meta : None }
    }

    pub fn generate(&mut self, rng : &mut StdRng, difficulty : f32) {
        let points = vec! [
            RoadPoint::new(0.0, 0.0, 0.0),
            //RoadPoint::new(10.0, 0.0, 0.0),
            //RoadPoint::new(30.0, 1.0, 0.2),
            //RoadPoint::new(35.0, 2.0, 0.2),
            //RoadPoint::new(60.0, 3.0, 0.0),
            RoadPoint::new(150.0, 0.0, 0.0)
        ];

        let heels = vec![
            Heel::new(0.0, 50.0, 0.0, 0.003),
            Heel::new(50.0, 100.0, 0.003, 0.0),
            //Heel::new(75.0, 100.0, -0.001, 0.0)
        ];

        let road_data = RoadData::new(0.0, 150.0, points, heels);

        let roads_data = vec![road_data];
        
        let meta = RoadPathMeta { roads_data, length : 100.0 };
        self.meta = Some(meta);
    }

    pub fn get_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }

    pub fn get_reverse_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }
}