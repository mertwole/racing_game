use std::rc::Rc;

use rand::{rngs::StdRng, Rng};

use crate::engine::billboards::*;
use crate::engine::traffic::*;
use crate::engine::track::*;
use crate::game::Game;

#[readonly::make]
pub struct RoadPath {
    pub source_id : usize,
    pub destination_id : usize,

    meta : Option<RoadPathMeta>
}

#[derive(Clone)]
pub struct RoadPathMeta{
    pub length : f32,
    pub track_data : TrackData,
    pub billboards : Billboards,
    pub traffic : Traffic
}

impl RoadPath {
    pub fn new(source_id : usize, destination_id : usize) -> RoadPath {
        RoadPath { source_id, destination_id, meta : None }
    }

    pub fn generate(&mut self, rng : &mut StdRng, billboard_factories : &Vec<BillboardFactory>, length : f32) {
        let mut curvatures : Vec<Curvature>= Vec::new();

        let start_straight_len = rng.gen_range(50.0, 100.0);
        let end_straight_len = rng.gen_range(10.0, 50.0);
        let mut curr_dist = start_straight_len;
        loop {
            let curvature_len = rng.gen_range(20.0, 40.0);
            let curvature_strength = rng.gen_range(-0.01, 0.01);

            curvatures.push( Curvature { start : curr_dist, end : curr_dist + curvature_len, strength : curvature_strength });

            curr_dist += curvature_len;

            let straight_len = rng.gen_range(1.0, 10.0);
            curr_dist += straight_len;

            if curr_dist + end_straight_len > length { break; }
        }

        let heels = vec![
            //Heel::new(0.0, 50.0, 0.0, 0.003),
            //Heel::new(50.0, 100.0, 0.003, 0.0),
            //Heel::new(75.0, 100.0, -0.001, 0.0)
        ];
        let mut roads = Vec::new();
        roads.push(Road::new(
            1.0, 
            vec![KeyPoint::new(0.0, 0.0), KeyPoint::new(10.0, 0.0), KeyPoint::new(30.0, 2.0), KeyPoint::new(50.0, 0.0), KeyPoint::new(length, 0.0)], 
            Rc::from(Game::load_image_rgb("road_tex.png"))
        ));
        let track_data = TrackData::new(length, curvatures, heels, roads);
        
        let mut billboards = Billboards::new();
        billboards.add_static(billboard_factories[0].construct(40.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(60.0, -1.1));
        billboards.add_static(billboard_factories[0].construct(80.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(83.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(86.0, 1.1));
        billboards.add_static(billboard_factories[0].construct(89.0, 1.1));

        let mut traffic = Traffic::new(1.0);
        let traffic_car_billboard = BillboardFactory::new(&Game::load_image_rgba("test_spritesheet.png"), Game::load_file("test_spritesheet.meta"));
        let car = TrafficCar::new(traffic_car_billboard.construct(10.0, 0.0), 0.5, 1.0, 1.0);
        traffic.add_car(&mut billboards, car);

        let meta = RoadPathMeta { track_data : track_data, length, billboards, traffic };
        self.meta = Some(meta);
    }

    pub fn get_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }

    pub fn get_reverse_meta(&self) -> RoadPathMeta{
        self.meta.clone().unwrap()
    }
}