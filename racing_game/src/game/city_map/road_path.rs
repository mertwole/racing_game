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
        let straight_segments = vec![
            StraightSegment::new(0.0, 20.0, 0.0),
            StraightSegment::new(50.0, 100.0, 4.5),
        ];

        let curved_segments = vec![
            CurvedSegment::new(20.0, 50.0, 0.0, 4.5),
        ];

        let heels = vec![
            //Heel::new(0.0, 25.0, 0.0, 0.001),
            //Heel::new(25.0, 75.0, 0.001, -0.001),
            //Heel::new(75.0, 100.0, -0.001, 0.0)
        ];

        let road_data = RoadData::new(0.0, 100.0, curved_segments, straight_segments, heels);

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