use crate::image::{RgbImage};

mod camera;
mod car;
mod math;
mod road;
mod billboards;

use crate::render::*;
use crate::window::*;
use crate::input::*;

use road::*;
use camera::*;
use car::*;
use billboards::*;

pub struct Game {
    screen_width : u32,
    screen_height : u32,

    window : Window,
    render : Render,

    input : Input,

    camera : Camera,

    road : Road,
    car : Car,
    billboards : Billboards
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

        let billboards = Billboards::new();

        Game { window, render, input, camera, road, car, screen_width, screen_height, billboards }
    }

    pub fn enter_gameloop(&mut self) {
        loop {
            let delta_time = self.window.get_time();
            self.window.set_time(0.0);
            
            println!("FPS : {}", 1.0 / delta_time);

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

        self.camera.road_distance += self.car.get_speed() * delta_time;
    }

    fn render(&mut self, mut buffer : RgbImage) {
        self.road.render_from_y_data(&mut buffer, &self.camera);

        self.car.render(&mut buffer);

        self.billboards.render(&self.camera, &self.road.y_data, &mut buffer, 150.0);

        self.render.render(&mut self.window, buffer);
    }
}