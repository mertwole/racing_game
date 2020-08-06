use image::{RgbImage, RgbaImage};

use super::city_map::road_path::RoadPathMeta;
use super::{Game};
use crate::engine::billboards::*;
use crate::engine::road::*;
use crate::engine::horizon::*;
use crate::engine::camera::*;
use crate::engine::common::{IVec2, ImageOps};
use super::{EventType, InputEvent};

mod car;
use car::*;

pub struct Ride {
    roads : Vec<Road>,
    billboards : Billboards,
    length : f32,
    horizon : Horizon,
    camera : Camera,
    active : bool,

    car : Car,
    car_input : IVec2
}

pub enum RideEvent {
    Finished
}

const screen_height : u32 = 360;
const screen_width : u32 = 640;

impl Ride {
    pub fn new() -> Ride {
        let horizon = Horizon::new(Game::load_image_rgba("horizon.png"));
        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0, x_offset : 0.0 }; 
        let car_img = Game::load_image_rgba("ferrari.png");
        let car_width = car_img.width() as f32 / screen_width as f32;
        let car = Car::new(car_img, car_width, 5.0, 5.0, 10.0, 1.5);
        Ride { roads : Vec::new(), billboards : Billboards::new(), car, horizon, camera, length : 0.0, active : false, car_input : IVec2::zero() }
    }

    pub fn start_ride(&mut self, ride_data : RoadPathMeta) {
        self.active = true;
        self.camera.road_distance = 0.0;
        self.length = ride_data.length;

        self.billboards = ride_data.billboards;
        for road_data in ride_data.roads_data {
            self.roads.push(Road::new(Game::load_image_rgb("road_tex.png"), road_data));
        }
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        if !self.active { return; } 

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::Gas, EventType::Pressed) => { self.car_input.y = 1; }
                (InputEvent::Gas, EventType::Released) => { self.car_input.y = -1; }

                (InputEvent::Left, EventType::Pressed) => { self.car_input.x = -1; }
                (InputEvent::Left, EventType::Released) => { self.car_input.x = 0; }

                (InputEvent::Right, EventType::Pressed) => { self.car_input.x = 1; }
                (InputEvent::Right, EventType::Released) => { self.car_input.x = 0; }
                _ => { }
            }
        }
    }

    pub fn update(&mut self, delta_time : f32) -> Vec<RideEvent> {
        if !self.active { return Vec::new(); } 

        for road in &mut self.roads {
            road.compute_y_data(&self.camera, screen_height);
        }
        
        if self.car_input.y == 1 {
            self.car.gas(delta_time);
        } else if self.car_input.y == -1 {
            self.car.brake(delta_time);
        }

        self.car.steer(self.car_input.x as f32, delta_time);

        self.car.x_pos -= self.roads[0].get_horz_speed(&self.camera) * self.car.speed * delta_time * 5.0;

        let road_bounds = self.roads[0].get_bounds();
        if self.car.x_pos - self.car.width * 0.5 < road_bounds.0 {
            println!("out of bounds!");
        }
        if self.car.x_pos + self.car.width * 0.5 > road_bounds.1 {
            println!("out of bounds!");
        }

        self.camera.x_offset = self.car.x_pos;
        self.camera.road_distance += self.car.speed * delta_time;

        if self.camera.road_distance >= self.length {
            self.active = false;
            return vec![RideEvent::Finished];
        }

        return Vec::new();
    }

    pub fn render(&self, buffer : &mut RgbImage) {
        if !self.active { return; } 
        
        self.horizon.render(100, 0.0, buffer);
        for road in &self.roads { road.render_from_y_data(buffer, &self.camera); }
        self.car.render(buffer);
        self.billboards.render_all(&self.camera, &self.roads[0].y_data, buffer);
    }
}