use std::collections::HashMap;

use image::{RgbImage, RgbaImage};

use crate::engine::common::{IVec2, ImageOps, Math};
use crate::game::*;

#[derive(Clone)]
pub struct Characteristics {
    pub acceleration : f32,
    pub deceleration : f32,
    pub brake_deceleration : f32,
    pub max_speed : f32,
    pub steer_speed : f32,
    pub gas_mileage : f32
}

impl Characteristics {
    pub fn new(acceleration : f32, deceleration : f32, brake_deceleration : f32, max_speed : f32, steer_speed : f32, gas_mileage : f32) -> Characteristics {
        Characteristics {
            acceleration,
            deceleration,
            brake_deceleration,
            max_speed,
            steer_speed,
            gas_mileage
        }
    }
}

pub struct Car{
    base_characteristics : Characteristics,
    characteristics : Characteristics,
    damage : Damage,

    pub speed : f32,
    pub roadside_dist : Option<f32>,
    pub x_pos : f32,
    pub width : f32,
    image : RgbaImage,

    input_horz : Option<InputEvent>,
    prev_input_horz : Option<InputEvent>,

    input_vert : Option<InputEvent>
}

impl Car {
    pub fn new(image : RgbaImage, width : f32, characteristics : Characteristics) -> Car {
        Car { 
            base_characteristics : characteristics.clone(),
            characteristics,
            damage : Damage::void(),

            speed : 0.0,
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

    pub fn fix_system(&mut self, system : CarSystem, percent : Percent) {
        self.damage.car_systems.get_mut(&system).unwrap().0 += percent.0;
    }

    pub fn update(&mut self, delta_time : f32) {
        let steer = match self.input_horz {
            Some(InputEvent::CarLeft) => { -1.0 }
            Some(InputEvent::CarRight) => { 1.0 }
            _ => { 0.0 }
        };

        self.x_pos += steer * delta_time * self.characteristics.steer_speed * (self.speed / self.characteristics.max_speed);

        let acceleration = match self.input_vert {
            Some(InputEvent::CarGas) => { self.characteristics.acceleration }
            Some(InputEvent::CarBrake) => { -self.characteristics.brake_deceleration }
            _ => { -self.characteristics.deceleration }
        };
    
        self.speed += delta_time * acceleration;
        
        let max_speed = self.characteristics.max_speed * 
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

enum Characteristic {
    Acceleration,
    Deceleration,
    BrakeDeceleration,
    MaxSpeed,
    SteerSpeed,
    GasMileage
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum CarSystem {
    Wheels,
    Transmission,
    Chase,
    Engine,
    Brake,
    Starter
}

struct DamageEffect {
    characteristic : Characteristic,
    car_system : CarSystem,
    multiplier_when_fully_broken : f32
}

impl DamageEffect {
    fn new(characteristic : Characteristic, car_system : CarSystem, multiplier_when_fully_broken : f32) -> DamageEffect {
        DamageEffect { characteristic, car_system, multiplier_when_fully_broken }
    }

    fn apply(&self, characteristicts : &mut Characteristics, damage : &Damage) {
        let multiplier = Math::lerp(1.0, self.multiplier_when_fully_broken, damage.car_systems.get(&self.car_system).unwrap().to_norm());
        match self.characteristic {
            Characteristic::Acceleration => { characteristicts.acceleration *= multiplier; }
            Characteristic::Deceleration => { characteristicts.deceleration *= multiplier; }
            Characteristic::BrakeDeceleration => { characteristicts.brake_deceleration *= multiplier; }
            Characteristic::MaxSpeed => { characteristicts.max_speed *= multiplier; }
            Characteristic::SteerSpeed => { characteristicts.steer_speed *= multiplier; }
            Characteristic::GasMileage => { characteristicts.gas_mileage *= multiplier; }
        }
    }
}

struct Damage {
    car_systems : HashMap<CarSystem, Percent>,
    effects : Vec<DamageEffect>
}

impl Damage {
    fn void () -> Damage {
        let mut car_systems = HashMap::<CarSystem, Percent>::new();
        
        car_systems.insert(CarSystem::Brake, Percent(100.0));
        car_systems.insert(CarSystem::Chase, Percent(100.0));
        car_systems.insert(CarSystem::Engine, Percent(100.0));
        car_systems.insert(CarSystem::Starter, Percent(100.0));
        car_systems.insert(CarSystem::Transmission, Percent(100.0));
        car_systems.insert(CarSystem::Wheels, Percent(100.0));

        let mut effects = vec![
            DamageEffect::new(Characteristic::Deceleration, CarSystem::Brake, 0.5)
        ];

        Damage { car_systems, effects }
    }

    fn affect_characteristics(&self, mut characteristics : Characteristics) -> Characteristics {
        for effect in &self.effects { effect.apply(&mut characteristics, &self); }
        characteristics
    }
}