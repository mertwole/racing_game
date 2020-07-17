use crate::image::{RgbaImage, RgbImage, Rgb};

pub struct Car {
    pub speed : f32,
    image : RgbaImage
}

impl Car {
    pub fn new(image : RgbaImage) -> Car {
        Car { image, speed : 0.0 }
    }

    pub fn render(&self, image : &mut RgbImage) {
        let render_x = image.width() / 2 - self.image.width() / 2;
        let render_y = 0;

        for x in 0..self.image.width() {
            for y in 0..self.image.height() {
                let car_pixel = self.image.get_pixel(x, self.image.height() - y - 1);
                if car_pixel[3] != 0{
                    image.put_pixel(render_x + x, render_y + y, Rgb([car_pixel[0], car_pixel[1], car_pixel[2]]))
                }
            }
        }
    }
}