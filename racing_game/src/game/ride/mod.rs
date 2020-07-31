use image::RgbImage;

use super::city_map::road_path::RoadPathMeta;
use super::{Game, car::Car};
use crate::engine::road::*;
use crate::engine::horizon::*;
use crate::engine::camera::*;
use super::{EventType, InputEvent};

pub struct Ride {
    roads : Vec<Road>,
    length : f32,
    horizon : Horizon,
    camera : Camera,
    car : Car,
    active : bool
}

pub enum RideEvent {
    Finished
}

const screen_height : u32 = 360;

impl Ride {
    pub fn new() -> Ride {
        //let road = Road::new(Game::load_image_rgb("road_tex.png"));
            
        let car = Car::new(Game::load_image_rgba("ferrari.png"), 5.0, 5.0, 10.0);

        let horizon = Horizon::new(Game::load_image_rgba("horizon.png"));

        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0 }; 

        Ride { roads : Vec::new(), car, horizon, camera, length : 0.0, active : false }
    }

    pub fn start_ride(&mut self, ride_data : RoadPathMeta) {
        self.active = true;
        // TODO : load road, horizon, bollboards, e.t.c here.
        self.car = Car::new(Game::load_image_rgba("ferrari.png"), 5.0, 5.0, 10.0);
        self.camera.road_distance = 0.0;

        self.length = ride_data.length;

        for road_data in ride_data.roads_data {
            self.roads.push(Road::new(Game::load_image_rgb("road_tex.png"), road_data));
        }
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        if !self.active { return; } 

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::Gas, EventType::Pressed) => { self.car.gas(0.3f32); }
                _ => { }
            }
        }
    }

    pub fn update(&mut self, delta_time : f32) -> Vec<RideEvent> {
        if !self.active { return Vec::new(); } 

        for road in &mut self.roads {
            road.compute_y_data(&self.camera, screen_height);
        }
        //self.billboards.get_dynamic_mut(BillboardId(0)).road_distance += delta_time * 0.3;
        self.camera.road_distance += self.car.get_speed() * delta_time;

        if self.camera.road_distance >= self.length {
            self.active = false;
            return vec![RideEvent::Finished];
        }

        return Vec::new();
    }

    pub fn render(&self, buffer : &mut RgbImage) {
        if !self.active { return; } 
        
        self.horizon.render(100, 0.0, buffer);
        for road in &self.roads {
            road.render_from_y_data(buffer, &self.camera);
        }
        self.car.render(buffer);
        //self.billboards.render_all(&self.camera, &self.road.y_data, &mut buffer, 150.0);
    }
}