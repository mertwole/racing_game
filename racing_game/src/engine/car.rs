use crate::image::{RgbaImage, RgbImage};
use crate::engine::common::{IVec2, ImageOps};

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

        ImageOps::overlay_rgba(image, &self.image, &IVec2::new(render_x as isize, render_y));
    }
}