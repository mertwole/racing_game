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

pub struct Game {
    screen_width : u32,
    screen_height : u32,

    window : Window,
    render : Render,
    input : Input<InputEvent>,

    player : Player,

    pub city_map : CityMap,
    ui : UI,

    ride : Ride 
}

#[derive(Copy, Clone)]
pub enum InputEvent{
    Gas,
    Left,
    Right,

    UIRight,
    UILeft,
    UIUp,
    UIDown,
    UISelect,
    UIBack
}

#[derive(Clone)]
pub struct Percent(f32);

impl Game {
    pub fn new() -> Game {
        let screen_width = 640;
        let screen_height = 360;

        let window = Window::open(WindowParameters { width : screen_width, height : screen_height, title : String::from("title")});
        let render = Render::new(screen_width, screen_height);

        let mut input = Input::<InputEvent>::new();
        input.bind_action(InputEvent::Gas, Key::Up);
        input.bind_action(InputEvent::Left, Key::Left);
        input.bind_action(InputEvent::Right, Key::Right);

        input.bind_action(InputEvent::UIUp, Key::Up);
        input.bind_action(InputEvent::UIDown, Key::Down);
        input.bind_action(InputEvent::UILeft, Key::Left);
        input.bind_action(InputEvent::UIRight, Key::Right);
        input.bind_action(InputEvent::UISelect, Key::Enter);
        input.bind_action(InputEvent::UIBack, Key::Backspace);

        let mut generation_rng = rand::rngs::StdRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]);
        let parameters = city_map::GenerationParameters { 
            city_count : 9, 
            size : IVec2::new(200, 200),
            min_distance_between_cities : 50.0 
        };
        let city_map = CityMap::generate(&mut generation_rng, parameters);
        
        let ride = Ride::new();

        let ui = UI::new(&IVec2::new(screen_width as isize, screen_height as isize));

        let player = Player::new();

        Game { window, render, input, screen_width, screen_height, city_map, ride, ui, player }
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

            let render_buffer = RgbImage::new(self.screen_width, self.screen_height);
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
                    self.ride.start_ride(self.city_map.get_current_road_meta()); 
                }
                UIEvent::SelectCityDestination(destination) => {
                    self.city_map.set_city_destination(destination);
                }
                UIEvent::ChangePlayer(player) => {
                    self.player = player;
                }
                UIEvent::ServiceAction(action) => {
                    self.city_map.process_service_action(action, &mut self.player); 
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