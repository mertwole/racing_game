use super::billboards::*;
use super::camera::*;

#[derive(Clone)]
struct TrafficCar {
    billboard_id : BillboardId,
    speed : f32,
    road_distance : f32,
    sleeping : bool
}

#[derive(Clone)]
pub struct Traffic {
    cars : Vec<TrafficCar>
}

impl Traffic {
    pub fn new() -> Traffic {
        Traffic { cars : Vec::new() }
    }

    pub fn add_car(&mut self, billboard : Billboard, speed : f32, billboards : &mut Billboards) {   
        let road_distance = billboard.road_distance; 
        let billboard_id = billboards.add_dynamic(billboard);
        let car = TrafficCar { billboard_id, speed, road_distance, sleeping : true }; 
        self.cars.push(car);
    }

    pub fn update(&mut self, camera : &Camera, delta_time : f32, billboards : &mut Billboards) {
        let camera_far = camera.road_distance + camera.far_plane;

        for car in &mut self.cars {
            if !car.sleeping {
                car.road_distance += car.speed * delta_time;
                billboards.get_dynamic_mut(car.billboard_id).road_distance = car.road_distance;
            } else if car.road_distance < camera_far {
                car.sleeping = false;
            }
        }
    }
}