extern crate image;
use image::*;

mod render;
mod road;
mod camera;

use render::*;
use road::*;
use camera::*;

fn main() {
    let screen_width = 640;
    let screen_height = 480;

    let mut render = Render::new(screen_width, screen_height);
    let mut camera = Camera { screen_dist : 1.0, viewport_height : 1.0, y_pos : 1.5, far_plane : 100.0, pitch : 1.5, road_distance : 0.0 };  
    let road = Road::new();

    loop{
        let mut image = RgbImage::new(screen_width, screen_height);
        camera.road_distance += 0.2;
        road.render(&mut image, &camera);
        if !render.render(image) { return; }
    }
}
