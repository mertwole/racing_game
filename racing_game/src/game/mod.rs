use std::rc::Rc;

extern crate include_dir;
extern crate rand;
extern crate readonly;

use rand::*;
use crate::image::{RgbImage, RgbaImage};
use include_dir::{include_dir, Dir};

use crate::engine::input::*;
use crate::engine::render::*;
use crate::engine::window::*;
use crate::engine::common::{IVec2};

mod city_map;
use city_map::*;

mod ride;
use ride::*;

mod player;
use player::*;

mod ui;
use ui::*;

pub const RESOURCES_DIR : Dir = include_dir!("./resources");
pub const SCREEN_RESOLUTION : IVec2 = IVec2 { x : 640, y : 360 };

pub struct Game {
    window : Window,
    render : Render,
    input : Input<InputEvent>,

    player : Player,

    pub city_map : CityMap,
    ui : UI,

    pub ride : Ride 
}

#[derive(Copy, Clone, PartialEq)]
pub enum InputEvent{
    CarGas,
    CarBrake,
    CarLeft,
    CarRight,

    UIRight,
    UILeft,
    UIUp,
    UIDown,
    UISelect,
    UIBack
}

#[derive(Clone)]
pub struct Percent(f32);

#[derive(Clone)]
pub struct Time { pub hr : u32, pub min : u32 }

impl Time {
    pub fn new(hr : u32, min : u32) -> Time { Time { hr, min } }
}

impl Game {
    pub fn new() -> Game {
        let window = Window::open(WindowParameters { width : SCREEN_RESOLUTION.x as u32, height : SCREEN_RESOLUTION.y as u32, title : String::from("title")});
        let render = Render::new(SCREEN_RESOLUTION.x as u32, SCREEN_RESOLUTION.y as u32);

        let mut input = Input::<InputEvent>::new();
        input.bind_action(InputEvent::CarGas, Key::Up);
        input.bind_action(InputEvent::CarLeft, Key::Left);
        input.bind_action(InputEvent::CarRight, Key::Right);
        input.bind_action(InputEvent::CarBrake, Key::Down);

        input.bind_action(InputEvent::UIUp, Key::Up);
        input.bind_action(InputEvent::UIDown, Key::Down);
        input.bind_action(InputEvent::UILeft, Key::Left);
        input.bind_action(InputEvent::UIRight, Key::Right);
        input.bind_action(InputEvent::UISelect, Key::Enter);
        input.bind_action(InputEvent::UIBack, Key::Backspace);

        let mut generation_rng = rand::rngs::StdRng::from_seed([10, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]);
        let parameters = city_map::GenerationParameters { 
            city_count : 19, 
            size : IVec2::new(300, 300),
            min_distance_between_cities : 50.0,
            road_length_multiplier : 2.0
        };
        let city_map = CityMap::generate(&mut generation_rng, parameters);
        
        let ride = Ride::new();

        let ui = UI::new(&SCREEN_RESOLUTION);

        let player = Player::new();

        Game { window, render, input, city_map, ride, ui, player }
    }
}

// Game loop.
impl Game {
    pub fn enter_gameloop(&mut self) {
        unsafe {
            self.ui.set_game(Rc::from_raw(self as *const Game));
        }

        loop {
            let delta_time = self.window.get_time();
            self.window.set_time(0.0);
            
            //println!("FPS : {}", 1.0 / delta_time);
            
            self.update(delta_time as f32);

            let render_buffer = RgbImage::new(SCREEN_RESOLUTION.x as u32, SCREEN_RESOLUTION.y as u32);
            if self.window.should_close() { break; }
            self.render(render_buffer);
        }
    }

    fn update(&mut self, delta_time : f32) {
        let input_queue = self.input.process(&mut self.window); 

        let ui_events = self.ui.update(&input_queue, delta_time);

        for event in ui_events {
            match event {
                UIEvent::StartRide => { 
                    self.ride.start_ride(self.city_map.get_current_road_meta(), self.player.clone()); 
                }
                UIEvent::SelectCityDestination(destination) => {
                    self.city_map.set_city_destination(destination);
                }
                UIEvent::ChangePlayer(player) => {
                    self.player = player;
                }
                UIEvent::ServiceAction(id, action) => {
                    self.city_map.process_service_action(id, action, &mut self.player); 
                }
                _ => { }
            } 
        }

        let ride_events = self.ride.update(delta_time);

        for event in ride_events {
            match event {
                RideEvent::Finished => { 
                    self.city_map.arrived_to_city();
                    self.ui.enter_city(); 
                }
                RideEvent::ChangePlayer(player) => {
                    self.player = player;
                }
                _ => { }
            } 
        }

        self.ride.process_input(&input_queue);
    }

    fn render(&mut self, mut buffer : RgbImage) {
        self.ride.render(&mut buffer);
        self.ui.render(&mut buffer);

        self.render.render(&mut self.window, buffer);
    }
}

// File loading.
impl Game {
    pub fn load_image_rgb(name : &str) -> RgbImage {
        let file = RESOURCES_DIR.get_file(name);
        match file {
            Some(file) => { image::load_from_memory(file.contents()).unwrap().to_rgb() } 
            None => { panic!("file {} not found!", name); }
        }
    }

    pub fn load_image_rgba(name : &str) -> RgbaImage {
        let file = RESOURCES_DIR.get_file(name);
        match file {
            Some(file) => { image::load_from_memory(file.contents()).unwrap().to_rgba() } 
            None => { panic!("file {} not found!", name); }
        }
    }

    pub fn load_file<'a>(name : &str) -> &'a [u8] { 
        let file = RESOURCES_DIR.get_file(name);
        match file {
            Some(file) => { file.contents() } 
            None => { panic!("file {} not found!", name); }
        }
    }
}