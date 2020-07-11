extern crate image;
use image::*;

mod render;
use render::*;

struct Camera{
    dist : f32,
    viewport_height : f32,
    ground_height : f32
}

fn main() {
    let screen_width = 640;
    let screen_height = 480;

    let camera = Camera { dist : 1.0, viewport_height : 1.0, ground_height : 2.0 };

    let mut render = Render::new(screen_width, screen_height);

    loop{
        let mut image = RgbImage::new(screen_width, screen_height);

        let road_width = 0.9;

        for y in 0..screen_height {
            let y_norm = (y as f32) / (screen_height as f32);
            let screen_point_ground_height = camera.ground_height - (1.0 - y_norm) * camera.viewport_height; 
            let dist_to_road = screen_point_ground_height * camera.dist / (camera.ground_height - screen_point_ground_height);

            let screen_space_road_width = road_width * (camera.dist / (dist_to_road + 0.99));

            let pixel_road_width = ((screen_width as f32) * screen_space_road_width) as i32;
            let left_road_border = (screen_width as i32 - pixel_road_width) as i32 / 2i32;
            let right_road_border = screen_width as i32 - left_road_border;

            for x in 0..screen_width { 
                image.put_pixel(x, y, Rgb([if (x as i32) < left_road_border || (x as i32) > right_road_border { 100 } else { 255 } , 0, if (dist_to_road) as u32 % 2 == 1 { 100 } else { 255 }]));
            }
        }

        if !render.render(image) { return; }
    }
}
