use std::rc::Rc;

use image::{RgbImage, RgbaImage};

use super::city_map::road_path::RoadPathMeta;
use super::{Game, Player, SCREEN_RESOLUTION};
use crate::engine::billboards::*;
use crate::engine::track::*;
use crate::engine::horizon::*;
use crate::engine::camera::*;
use crate::engine::traffic::*;
use crate::engine::common::{IVec2, ImageOps};
use super::{EventType, InputEvent};

pub mod car;
use car::*;

pub struct Ride {
    track : Option<Track>,
    billboards : Billboards,
    length : f32,
    horizon : Horizon,
    traffic : Option<Traffic>,
    camera : Camera,
    active : bool,
    paused : bool,

    pub car : Car,
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
        let car_characteristics = Characteristics::new(5.0, 1.0, 10.0, 10.0, 1.5, 1.0);
        let car = Car::new(car_img, car_width, car_characteristics);

        Ride { 
            track : None,
            billboards : Billboards::new(), 
            car, 
            horizon, 
            camera,
            length : 0.0, 
            active : false,
            paused : false,
            player : None,
            traffic : None
        }
    }

    pub fn set_paused(&mut self, paused : bool) {
        self.paused = paused;
    }

    pub fn start_ride(&mut self, mut ride_data : RoadPathMeta, player : Player) {
        self.active = true;
        self.camera.road_distance = 0.0;
        self.length = ride_data.length;

        self.billboards = ride_data.billboards;
        self.track = Some(Track::new(ride_data.track_data));

        self.player = Some(player);
        let track_rc = unsafe { Rc::from_raw(self.track.as_ref().unwrap() as *const Track) };
        ride_data.traffic.set_track(track_rc);
        self.traffic = Some(ride_data.traffic);

        self.car.reset();
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        if !self.active || self.paused { return; } 
        self.car.process_input(input);
    }

    pub fn update(&mut self, delta_time : f32) -> Vec<RideEvent> {
        if !self.active || self.paused { return Vec::new(); } 

        self.car.update(delta_time);

        self.traffic.as_mut().unwrap().update(&self.camera, delta_time, &mut self.billboards);

        let mut events : Vec<RideEvent> = Vec::new();

        self.track.as_mut().unwrap().compute_y_data(&self.camera, SCREEN_RESOLUTION.y as u32);

        self.player.as_mut().unwrap().gas_level -= self.car.speed * delta_time;
        events.push(RideEvent::ChangePlayer(self.player.as_ref().unwrap().clone()));

        self.car.x_pos -= self.track.as_ref().unwrap().get_horz_speed(&self.camera) * self.car.speed * delta_time * 5.0;

        let car_left = self.car.x_pos - self.car.width * 0.5;
        let car_right = self.car.x_pos + self.car.width * 0.5;
        self.car.roadside_dist = self.track.as_ref().unwrap().roadside_dist(car_left, car_right, self.camera.road_distance + self.camera.screen_dist);

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
        self.track.as_ref().unwrap().render_from_y_data(buffer, &self.camera);
        self.billboards.render_all(&self.camera, &self.track.as_ref().unwrap().y_data, buffer);
        self.car.render(buffer);
    }
}