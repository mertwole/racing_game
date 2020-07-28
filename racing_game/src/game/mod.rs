use std::fs::File;
use std::rc::Rc;

extern crate rand;

use rand::*;
use crate::image::{RgbImage, RgbaImage, Rgb};

use crate::engine::*;
use crate::engine::math::IVec2;
use crate::engine::ui_controls::*;

mod city_map;
use city_map::*;

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

    city_map : CityMap,

    test_ui : UIPage
}

impl Game {
    pub fn new() -> Game {
        let screen_width = 640;
        let screen_height = 480;

        let window = Window::open(WindowParameters { width : screen_width, height : screen_height, title : String::from("title")});
        let render = Render::new(screen_width, screen_height);
        let input = Input::new();
        
        let camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0 };  
        
        let road_tex = image::open("resources/road_tex.png").unwrap().to_rgb();
        let road = Road::new(road_tex);
            
        let car_image = image::open("resources/ferrari.png").unwrap().to_rgba();
        let car = Car::new(car_image, 5.0, 5.0, 10.0);

        let spritesheet = image::open("resources/test_spritesheet.png").unwrap().to_rgba();
        let meta_file = File::open("resources/test_spritesheet.meta").unwrap(); 
        let mut billboards = Billboards::new();
        let car_billboard_factory = BillboardFactory::new(&spritesheet, &meta_file);

        billboards.add_dynamic(car_billboard_factory.construct(10.0, 0.5));
        billboards.add_static(car_billboard_factory.construct(13.0, -0.5));
        billboards.add_dynamic(car_billboard_factory.construct(12.0, 0.7));
        //billboards.add_static(car_billboard_factory.construct(15.0, -0.5));
        billboards.add_dynamic(car_billboard_factory.construct(14.0, 0.9));
        //billboards.add_static(car_billboard_factory.construct(11.0, -0.5));
        billboards.add_dynamic(car_billboard_factory.construct(16.0, 0.5));

        let horizon_image = image::open("resources/horizon.png").unwrap().to_rgba();
        let horizon = Horizon::new(horizon_image);

        let mut generation_rng = rand::rngs::StdRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]);
        let parameters = city_map::GenerationParameters { 
            city_count : 9, 
            grid_size : IVec2::new(20, 20),
            min_distance_between_cities : 2.0 
        };
        let city_map = CityMap::generate(&mut generation_rng, parameters);

        // Test UI
        let font_tex = image::open("resources/font.png").unwrap().to_rgba();
        let font = Font::new(font_tex, IVec2::new(12, 12), String::from("ABCDEFGHIJ"));
        let font = Rc::from(font);
        let mut test_ui = UIPage::new(IVec2::new(screen_width as isize, screen_height as isize), font.clone());

        let test_image = image::open("resources/ferrari.png").unwrap().to_rgba();
        let test_image = Rc::from(test_image);

        let text = UIText::new(font.clone(), String::from("ABC"), IVec2::new(100, 100), Pivot::Center);
        let image = UIImage::new(test_image.clone(), IVec2::new(0, 0), Pivot::Center);

        test_ui.add_control(Box::from(text));
        test_ui.add_control(Box::from(image));

        Game { window, render, input, camera, road, car, screen_width, screen_height, billboards, horizon, city_map, test_ui }
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
        /*self.horizon.render(self.road.y_data.len() as u32 - 1, 0.0, &mut buffer);

        self.road.render_from_y_data(&mut buffer, &self.camera);

        self.car.render(&mut buffer);

        self.billboards.render_all(&self.camera, &self.road.y_data, &mut buffer, 150.0);*/

        // Test UI.
        self.test_ui.draw(&mut buffer);

        self.render.render(&mut self.window, buffer);
    }
}