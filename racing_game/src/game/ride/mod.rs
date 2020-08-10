use image::{RgbImage, RgbaImage};

use super::city_map::road_path::RoadPathMeta;
use super::{Game, Player, SCREEN_RESOLUTION};
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
    player : Option<Player>
}

pub enum RideEvent {
    Finished,
    ChangePlayer(Player)
}

impl Ride {
    pub fn new() -> Ride {
        let horizon = Horizon::new(Game::load_image_rgba("horizon.png"));
        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0, x_offset : 0.0 }; 
        let car_img = Game::load_image_rgba("ferrari.png");
        let car_width = car_img.width() as f32 / SCREEN_RESOLUTION.x as f32;
        let car = Car::new(car_img, car_width, 5.0, 1.0, 10.0, 10.0, 1.5);
        Ride { 
            roads : Vec::new(),
            billboards : Billboards::new(), 
            car, 
            horizon, 
            camera,
            length : 0.0, 
            active : false,
            player : None
        }
    }

    pub fn start_ride(&mut self, ride_data : RoadPathMeta, player : Player) {
        self.active = true;
        self.camera.road_distance = 0.0;
        self.length = ride_data.length;

        self.player = Some(player);

        self.car.reset();

        self.billboards = ride_data.billboards;
        self.roads.clear();
        for road_data in ride_data.roads_data {
            self.roads.push(Road::new(Game::load_image_rgb("road_tex.png"), road_data));
        }
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        if !self.active { return; } 

        self.car.process_input(input);
    }

    pub fn update(&mut self, delta_time : f32) -> Vec<RideEvent> {
        if !self.active { return Vec::new(); } 
        self.car.update(delta_time);

        let mut events : Vec<RideEvent> = Vec::new();

        for road in &mut self.roads {
            road.compute_y_data(&self.camera, SCREEN_RESOLUTION.y as u32);
        }

        self.player.as_mut().unwrap().gas_level -= self.car.speed * delta_time;
        events.push(RideEvent::ChangePlayer(self.player.as_ref().unwrap().clone()));

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

        return events;
    }

    pub fn render(&self, buffer : &mut RgbImage) {
        if !self.active { return; } 
        
        self.horizon.render(100, 0.0, buffer);
        for road in &self.roads { road.render_from_y_data(buffer, &self.camera); }
        self.car.render(buffer);
        self.billboards.render_all(&self.camera, &self.roads[0].y_data, buffer);
    }
}