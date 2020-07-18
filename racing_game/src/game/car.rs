use crate::image::{RgbaImage, RgbImage, Rgb};

pub struct Car {
    acceleration : f32,
    deceleration : f32,
    speed : f32,
    max_speed : f32,
    image : RgbaImage
}

impl Car {
    pub fn new(image : RgbaImage, acceleration : f32, deceleration : f32, max_speed : f32) -> Car {
        Car { image, speed : 0.0, acceleration, deceleration, max_speed }
    }

    pub fn gas(&mut self, delta_time : f32) {
        self.speed += delta_time * self.acceleration;
        if self.speed > self.max_speed { self.speed = self.max_speed; }
    }

    pub fn brake(&mut self, delta_time : f32) {
        self.speed -= delta_time * self.deceleration;
        if self.speed < 0.0 { self.speed = 0.0; }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
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