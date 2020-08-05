use crate::engine::common::*;

#[derive(Clone)]
pub struct Heel{
    start : f32,
    end : f32,
    start_steepness : f32,
    end_steepness : f32
}

impl Heel {
    pub fn new(start : f32, end : f32, start_steepness : f32, end_steepness : f32) -> Heel {
        Heel { start, end, start_steepness, end_steepness }
    }

    fn get_width_multiplier(&self, distance : f32) -> Option<f32> {
        if distance < self.start || distance > self.end { return None; }

        let t = (distance - self.start) / (self.end - self.start);
        return Some(Math::lerp(self.start_steepness, self.end_steepness, t));
    }
}

#[derive(Clone)]
pub struct RoadData {
    track_length : f32,
    start_distance : f32,
    heels : Vec<Heel>,
    curvatures : Vec<Curvature>

}

#[derive(Clone)]
pub struct Curvature {
    pub start : f32,
    pub end : f32,
    pub strength : f32
}

pub enum OffsetMode {
    Normal(f32, usize),
    AsIs
}

impl RoadData {
    pub fn new(start_distance : f32, length : f32, curvatures : Vec<Curvature>, heels : Vec<Heel>) -> RoadData {
        RoadData { track_length : length, start_distance, heels, curvatures }
    }

    pub fn get_segment_offset(&self, camera_road_distance : f32, road_distance : f32) -> OffsetMode {
        for i in 0..self.curvatures.len() {
            if self.curvatures[i].start < road_distance && self.curvatures[i].end > road_distance {
                let dist = road_distance - Math::max(self.curvatures[i].start, camera_road_distance);
                return OffsetMode::Normal(dist * dist * self.curvatures[i].strength, i);
            }
        }

        return OffsetMode::AsIs;
    }

    pub fn get_hill_width_multiplier_delta(&self, road_distance : f32) -> f32 {
        for heel in &self.heels {
            if let Some(mul) = heel.get_width_multiplier(road_distance) {
                return mul;
            }
        }
        
        0.0
    }

    pub fn is_visible(&self, vis_road_dist : f32) -> bool {
        vis_road_dist >= self.start_distance && vis_road_dist <= self.start_distance + self.track_length
    }

    pub fn get_camera_pitch_delta(&self, camera_road_dist : f32) -> f32 {
        return self.get_hill_width_multiplier_delta(camera_road_dist) * 50.0;
    }
}