use super::billboards::*;
use super::camera::*;

pub struct TrafficCar {
    billboard_id : BillboardId,
    speed : f32,
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
        let billboard_id = self.car_billboards.add_dynamic(billboard);
        let car = TrafficCar { billboard_id, speed, sleeping : true }; 
        self.cars.push(car);
    }

    pub fn update(&mut self, camera : &Camera, delta_time : f32) {
        for car in &self.cars {

        }
    }

    pub fn render(&self, camera : &Camera) {
        //self.car_billboards.render_all(, y_data: &Vec<RoadYData>, buffer: &mut RgbImage)
    }
}