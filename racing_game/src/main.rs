extern crate image;
use image::*;

mod render;
mod road;
mod camera;
mod car;
mod window;
mod input;
mod math;

use render::*;
use road::*;
use camera::*;
use car::*;
use window::*;
use input::*;

fn main() {
    let screen_width = 640;
    let screen_height = 480;

    let mut window = Window::open(WindowParameters { width : screen_width, height : screen_height, title : String::from("title")});
    let mut render = Render::new(screen_width, screen_height);
    let mut input = Input::new();
    
    let mut camera = Camera { screen_dist : 2.0, viewport_height : 1.0, y_pos : 3.0, far_plane : 150.0, pitch : 1.5, road_distance : 0.0 };  
    let road = Road::new();
        
    let car_image = image::open("resources/ferrari.png").unwrap().to_rgba();
    let mut car = Car::new(car_image);

    loop{
        let mut image = RgbImage::new(screen_width, screen_height);

        // Camera movement.
        input.process(&mut window);
        if input.get_vertical() == 1 {
            car.speed += 0.01;
            if car.speed > 0.5 { car.speed = 0.5 }
        } else if input.get_vertical() < 1 {
            car.speed -= 0.01;
            if car.speed < 0.0 { car.speed = 0.0 }
        }
        camera.road_distance += car.speed;

        road.render(&mut image, &camera);
        car.render(&mut image);
        if !render.render(&mut window, image) { return; }
    }
}
