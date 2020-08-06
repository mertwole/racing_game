use rand::{RngCore, rngs::StdRng};

use crate::engine::billboards::*;
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
    pub roads_data : Vec<RoadData>,
    pub billboards : Billboards
}

impl RoadPath {
    pub fn new(source_id : usize, destination_id : usize) -> RoadPath {
        RoadPath { source_id, destination_id, meta : None }
    }

    pub fn generate(&mut self, rng : &mut StdRng, billboard_factories : &Vec<BillboardFactory>, length : f32) {
        let curvatures = vec![
            Curvature { start : 10.0, end : 30.0, strength : 0.01 },
            Curvature { start : 40.0, end : 60.0, strength : -0.01 }
        ];

        let heels = vec![
            //Heel::new(0.0, 50.0, 0.0, 0.003),
            //Heel::new(50.0, 100.0, 0.003, 0.0),
            //Heel::new(75.0, 100.0, -0.001, 0.0)
        ];

        let road_data = RoadData::new(0.0, 150.0, curvatures, heels);

        let roads_data = vec![road_data];
        
        let mut billboards = Billboards::new();
        billboards.add_static(billboard_factories[0].construct(40.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(60.0, -1.1));

        billboards.add_static(billboard_factories[0].construct(80.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(83.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(86.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(89.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(92.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(95.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(98.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(101.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(104.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(107.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(110.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(113.0, 1.1));

        billboards.add_static(billboard_factories[0].construct(80.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(83.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(86.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(89.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(92.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(95.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(98.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(101.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(104.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(107.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(110.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(113.0, -1.1));

        let meta = RoadPathMeta { roads_data, length : 150.0, billboards };
        self.meta = Some(meta);
    }

    pub fn get_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }

    pub fn get_reverse_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }
}