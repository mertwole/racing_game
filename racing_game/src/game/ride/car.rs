use image::{RgbImage, RgbaImage};

use crate::engine::common::{IVec2, ImageOps};

pub struct Car{
    acceleration : f32,
    deceleration : f32,
    pub speed : f32,
    max_speed : f32,
    steer_speed : f32,
    pub x_pos : f32,
    pub width : f32,
    image : RgbaImage,
}

impl Car {
    pub fn new(image : RgbaImage, width : f32, acceleration : f32, deceleration : f32, max_speed : f32, steer_speed : f32) -> Car {
        Car { 
            speed : 0.0, 
            acceleration, 
            deceleration, 
            max_speed,
            steer_speed,
            x_pos : 0.0,
            width,

            image
        }
    }

    pub fn gas(&mut self, delta_time : f32) {
        self.speed += delta_time * self.acceleration;
        if self.speed > self.max_speed { self.speed = self.max_speed; }
    }

    pub fn brake(&mut self, delta_time : f32) {
        self.speed -= delta_time * self.deceleration;
        if self.speed < 0.0 { self.speed = 0.0; }
    }

    pub fn steer(&mut self, direction : f32, delta_time : f32) {
        self.x_pos += direction * delta_time * self.steer_speed;
    }

    pub fn render(&self, image : &mut RgbImage) {
        let render_x = image.width() / 2 - self.image.width() / 2;
        let render_y = 0;

        ImageOps::overlay_rgba(image, &self.image, &IVec2::new(render_x as isize, render_y));
    }
}