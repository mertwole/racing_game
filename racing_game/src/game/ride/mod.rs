use image::{RgbImage, RgbaImage};

use super::city_map::road_path::RoadPathMeta;
use super::{Game};
use crate::engine::billboards::*;
use crate::engine::road::*;
use crate::engine::horizon::*;
use crate::engine::camera::*;
use crate::engine::common::{IVec2, ImageOps};
use super::{EventType, InputEvent};

pub struct Car{
    acceleration : f32,
    deceleration : f32,
    speed : f32,
    max_speed : f32,
    steer_speed : f32,
    x_pos : f32,
    image : RgbaImage
}

impl Car {
    pub fn new(image : RgbaImage, acceleration : f32, deceleration : f32, max_speed : f32, steer_speed : f32) -> Car {
        Car { 
            speed : 0.0, 
            acceleration, 
            deceleration, 
            max_speed,
            steer_speed,
            x_pos : 0.0,

            image
        }
    }

    pub fn gas(&mut self, delta_time : f32) {
        self.speed += delta_time * self.acceleration;
        if self.speed > self.max_speed { self.speed = self.max_speed; }
    }

    pub fn brake(&mut self, delta_time : f32) {
        self.speed -= delta_time * self.deceleration;
        if self.speed < 0.0 { self.speed = 0.0; }
    }

    pub fn steer(&mut self, direction : f32, delta_time : f32) {
        self.x_pos += direction * delta_time * self.steer_speed;
    }

    pub fn render(&self, image : &mut RgbImage) {
        let render_x = image.width() / 2 - self.image.width() / 2;
        let render_y = 0;

        ImageOps::overlay_rgba(image, &self.image, &IVec2::new(render_x as isize, render_y));
    }
}

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

impl Ride {
    pub fn new() -> Ride {
        let horizon = Horizon::new(Game::load_image_rgba("horizon.png"));
        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0, x_offset : 0.0 }; 
        let car = Car::new(Game::load_image_rgba("ferrari.png"), 5.0, 5.0, 10.0, 1.5);
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