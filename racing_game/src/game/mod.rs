extern crate include_dir;
extern crate rand;
extern crate readonly;

use rand::*;
use crate::image::{RgbImage, RgbaImage};
use include_dir::{include_dir, Dir};

use crate::engine::billboards::*;
use crate::engine::camera::*;
use crate::engine::car::*;
use crate::engine::horizon::*;
use crate::engine::input::*;
use crate::engine::render::*;
use crate::engine::road::*;
use crate::engine::window::*;
use crate::engine::common::{IVec2};

mod city_map;
use city_map::*;

mod ui_screen_manager;
use ui_screen_manager::*;

pub const RESOURCES_DIR : Dir = include_dir!("./resources");

pub struct Game {
    screen_width : u32,
    screen_height : u32,

    window : Window,
    render : Render,
    input : Input,

    camera : Camera,
    road : Road,
    car : Car,
    billboards : Billboards,
    horizon : Horizon,

    pub city_map : CityMap,

    screen_manager : UIScreenManager
}

impl Game {
    pub fn new() -> Game {
        let screen_width = 640;
        let screen_height = 360;

        let window = Window::open(WindowParameters { width : screen_width, height : screen_height, title : String::from("title")});
        let render = Render::new(screen_width, screen_height);
        let input = Input::new();
        
        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0 };  
        
        let road = Road::new(Game::load_image_rgb("road_tex.png"));
            
        let car = Car::new(Game::load_image_rgba("ferrari.png"), 5.0, 5.0, 10.0);
 
        let mut billboards = Billboards::new();
        let car_billboard_factory = BillboardFactory::new(&Game::load_image_rgba("test_spritesheet.png"), Game::load_file("test_spritesheet.meta"));

        billboards.add_dynamic(car_billboard_factory.construct(10.0, 0.5));
        billboards.add_static(car_billboard_factory.construct(13.0, -0.5));
        billboards.add_dynamic(car_billboard_factory.construct(12.0, 0.7));
        billboards.add_dynamic(car_billboard_factory.construct(14.0, 0.9));
        billboards.add_dynamic(car_billboard_factory.construct(16.0, 0.5));

        let horizon = Horizon::new(Game::load_image_rgba("horizon.png"));

        let mut generation_rng = rand::rngs::StdRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]);
        let parameters = city_map::GenerationParameters { 
            city_count : 9, 
            size : IVec2::new(200, 200),
            min_distance_between_cities : 50.0 
        };
        let city_map = CityMap::generate(&mut generation_rng, parameters);

        let screen_manager = UIScreenManager::new(&IVec2::new(screen_width as isize, screen_height as isize));

        Game { window, render, input, camera, road, car, screen_width, screen_height, billboards, horizon, city_map, screen_manager }
    }

    pub fn load_image_rgb(name : &str) -> RgbImage {
        image::load_from_memory(RESOURCES_DIR.get_file(name).unwrap().contents()).unwrap().to_rgb()
    }

    pub fn load_image_rgba(name : &str) -> RgbaImage {
        image::load_from_memory(RESOURCES_DIR.get_file(name).unwrap().contents()).unwrap().to_rgba()
    }

    pub fn load_file<'a>(name : &str) -> &'a [u8] { 
        RESOURCES_DIR.get_file(name).unwrap().contents()
    }

    pub fn enter_gameloop(&mut self) {
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
        self.road.compute_y_data(&self.camera, self.screen_height);

        self.input.process(&mut self.window);

        if self.input.get_vertical() == 1 {
            self.car.gas(delta_time);
        } else {
            self.car.brake(delta_time);
        }

        self.billboards.get_dynamic_mut(BillboardId(0)).road_distance += delta_time * 0.3;

        self.camera.road_distance += self.car.get_speed() * delta_time;
    }

    fn render(&mut self, mut buffer : RgbImage) {
        if self.screen_manager.is_game_visible() {
            self.horizon.render(self.road.y_data.len() as u32 - 1, 0.0, &mut buffer);
            self.road.render_from_y_data(&mut buffer, &self.camera);
            self.car.render(&mut buffer);
            self.billboards.render_all(&self.camera, &self.road.y_data, &mut buffer, 150.0);
        }

        self.screen_manager.render(&self, &mut buffer);

        self.render.render(&mut self.window, buffer);
    }
}