use image::{RgbImage, RgbaImage};

use crate::engine::common::{IVec2, ImageOps, Math};
use crate::game::*;

pub struct Car{
    acceleration : f32,
    deceleration : f32,
    brake_deceleration : f32,
    pub speed : f32,
    max_speed : f32,
    steer_speed : f32,
    pub roadside_dist : Option<f32>,
    pub x_pos : f32,
    pub width : f32,
    image : RgbaImage,

    input_horz : Option<InputEvent>,
    prev_input_horz : Option<InputEvent>,

    input_vert : Option<InputEvent>
}

impl Car {
    pub fn new(image : RgbaImage, width : f32, acceleration : f32, deceleration : f32, brake_deceleration : f32, max_speed : f32, steer_speed : f32) -> Car {
        Car { 
            speed : 0.0, 
            acceleration, 
            deceleration, 
            brake_deceleration,
            max_speed,
            steer_speed,
            roadside_dist : None,
            x_pos : 0.0,
            width,

            image,

            input_horz : None,
            prev_input_horz : None,

            input_vert : None
        }
    }

    pub fn reset(&mut self) {
        self.roadside_dist = None;
        self.input_horz = None;
        self.input_vert = None;
        self.prev_input_horz = None;
        self.speed = 0.0;
        self.x_pos = 0.0;
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match event_type {
                EventType::Pressed => { 
                    match event {
                        InputEvent::CarLeft | InputEvent::CarRight => { 
                            self.prev_input_horz = self.input_horz;
                            self.input_horz = Some(*event);
                        }
                        InputEvent::CarBrake | InputEvent::CarGas => { 
                            self.input_vert = Some(*event);
                        }
                        _ => { }
                    }
                }
                EventType::Released => { 
                    match event {
                        InputEvent::CarLeft | InputEvent::CarRight => { 
                            if Some(*event) == self.prev_input_horz { 
                                self.prev_input_horz = None; 
                            }
                            if Some(*event) == self.input_horz { 
                                self.input_horz = self.prev_input_horz; 
                                self.prev_input_horz = None; 
                            }
                        }
                        InputEvent::CarBrake | InputEvent::CarGas => { 
                            if Some(*event) == self.input_vert {
                                self.input_vert = None;
                            }
                        }
                        _ => { }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, delta_time : f32) {
        let steer = match self.input_horz {
            Some(InputEvent::CarLeft) => { -1.0 }
            Some(InputEvent::CarRight) => { 1.0 }
            _ => { 0.0 }
        };

        self.x_pos += steer * delta_time * self.steer_speed * (self.speed / self.max_speed);

        let acceleration = match self.input_vert {
            Some(InputEvent::CarGas) => { self.acceleration }
            Some(InputEvent::CarBrake) => { -self.brake_deceleration }
            _ => { -self.deceleration }
        };
    
        self.speed += delta_time * acceleration;
        
        let max_speed = self.max_speed * 
        if let Some(roadside_dist) = self.roadside_dist {
            1.0 / (roadside_dist.abs() * 3.0 + 1.0)
        } else {
            1.0
        };
        
        if self.speed > max_speed { self.speed = max_speed; }
        if self.speed < 0.0 { self.speed = 0.0 };
    }

    pub fn render(&self, image : &mut RgbImage) {
        let render_x = image.width() / 2 - self.image.width() / 2;
        let render_y = 0;

        ImageOps::overlay_rgba(image, &self.image, &IVec2::new(render_x as isize, render_y));
    }
}