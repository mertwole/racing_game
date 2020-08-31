use std::rc::Rc;

use super::common::Math;
use super::billboards::*;
use super::camera::*;
use super::track::*;

#[derive(Clone)]
pub struct TrafficCar {
    billboard_id : BillboardId,
    billboard : Option<Billboard>,
    speed : f32,
    steer_speed : f32,
    width : f32,
    road_distance : f32,
    x_pos : f32,
    sleeping : bool
}

impl TrafficCar {
    pub fn new(billboard : Billboard, width : f32, speed : f32, steer_speed : f32) -> TrafficCar {
        TrafficCar {
            billboard_id : BillboardId(0),
            speed,
            steer_speed,
            width,
            road_distance : billboard.road_distance,
            x_pos : billboard.offset,
            billboard : Some(billboard),
            sleeping : true
        }
    }
}

#[derive(Clone)]
pub struct Traffic {
    cars : Vec<TrafficCar>,
    track : Option<Rc<Track>>,
    overtake_distance : f32
}

impl Traffic {
    pub fn new(overtake_distance : f32) -> Traffic {
        Traffic { cars : Vec::new(), track : None, overtake_distance }
    }

    pub fn set_track(&mut self, track : Rc<Track>) {
        self.track = Some(track);
    }

    pub fn add_car(&mut self, billboards : &mut Billboards, mut car : TrafficCar) {  
        let billboard = car.billboard.take().unwrap(); 
        let road_distance = billboard.road_distance; 
        car.billboard_id = billboards.add_dynamic(billboard);
        self.cars.push(car);
    }

    pub fn update(&mut self, camera : &Camera, delta_time : f32, billboards : &mut Billboards) {
        for i in 0..self.cars.len() {
            if !self.cars[i].sleeping {
                let car_x = self.cars[i].x_pos;

                let mut closest_road = &self.track.as_ref().unwrap().data.roads[0];
                let mut closest_road_dist = std::f32::INFINITY;

                for road in &self.track.as_ref().unwrap().data.roads {
                    let road_offset = road.get_segment_offset(self.cars[i].road_distance);
                    if road_offset.is_none() { continue; }

                    let dist_to_road = (road_offset.unwrap() - car_x).abs();
                    if closest_road_dist > dist_to_road { 
                        closest_road_dist = dist_to_road;
                        closest_road = road;
                    }
                }

                let car_width = self.cars[i].width;
                let car_left = car_x - car_width * 0.5;
                let car_right = car_x + car_width * 0.5;
                let roadside_dist = closest_road.roadside_dist(car_left, car_right, self.cars[i].road_distance);
                if let Some(roadside_dist) = roadside_dist {
                    self.cars[i].x_pos += self.cars[i].steer_speed * delta_time * if roadside_dist > 0.0 { -1.0 } else { 1.0 };
                }

                self.cars[i].road_distance += self.cars[i].speed * delta_time;

                let car_billboard = billboards.get_dynamic_mut(self.cars[i].billboard_id);
                car_billboard.road_distance = self.cars[i].road_distance;
                car_billboard.offset = self.cars[i].x_pos;
            } else {
                if self.cars[i].speed > 0.0 && self.cars[i].road_distance <= camera.road_distance { 
                    self.cars[i].sleeping = false; 
                }
                if self.cars[i].speed < 0.0 && self.cars[i].road_distance <= camera.road_distance + camera.far_plane { 
                    self.cars[i].sleeping = false; 
                }
            }
        }
    }
}