use image::RgbImage;

use super::billboards::*;
use super::camera::*;
use super::road::*;

struct TrafficCar {
    billboard_id : BillboardId,
    speed : f32,
    road_distance : f32,
    sleeping : bool
}

pub struct Traffic {
    car_billboards : Billboards,
    cars : Vec<TrafficCar>
}

impl Traffic {
    pub fn new() -> Traffic {
        Traffic { 
            car_billboards : Billboards::new(), 
            cars : Vec::new() 
        }
    }

    pub fn add_car(&mut self, billboard : Billboard, speed : f32) {   
        let road_distance = billboard.road_distance; 
        let billboard_id = self.car_billboards.add_dynamic(billboard);
        let car = TrafficCar { billboard_id, speed, road_distance, sleeping : true }; 
        self.cars.push(car);
    }

    pub fn update(&mut self, camera : &Camera, delta_time : f32) {
        let camera_far = camera.road_distance + camera.far_plane;

        for car in &mut self.cars {
            if !car.sleeping {
                car.road_distance += car.speed * delta_time;
                self.car_billboards.get_dynamic_mut(car.billboard_id).road_distance = car.road_distance;
            } else if car.road_distance < camera_far {
                car.sleeping = false;
            }
        }
    }

    pub fn render(&self, camera : &Camera, y_data: &Vec<RoadYData>, buffer : &mut RgbImage) {
        self.car_billboards.render_all(camera, y_data, buffer);
    }
}