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
    points : Vec<RoadPoint>
}

#[derive(Clone)]
pub struct RoadPoint {
    road_distance : f32,
    offset : f32,
    angle : f32
}

impl RoadPoint {
    pub fn new(road_distance : f32, offset : f32, angle : f32) -> RoadPoint {
        RoadPoint { road_distance, offset, angle }
    }
}

impl RoadData {
    pub fn new(start_distance : f32, length : f32, road_points : Vec<RoadPoint>, heels : Vec<Heel>) -> RoadData {
        RoadData { track_length : length, start_distance, heels, points : road_points }
    }

    pub fn get_norm_segment_offset(&self, road_distance : f32) -> f32 {
        for i in 0..self.points.len() {
            if road_distance < self.points[i].road_distance {
                let start_angle = self.points[i - 1].angle;
                let end_angle = self.points[i].angle;
                let start_distance = self.points[i - 1].road_distance;
                let end_distance = self.points[i].road_distance;
                let start_offset = self.points[i - 1].offset;
                let end_offset = self.points[i].offset;

                if start_angle == end_angle {
                    let t = (road_distance - self.points[i - 1].road_distance) / (self.points[i].road_distance - self.points[i - 1].road_distance);
                    return Math::lerp(start_offset, end_offset, t);
                }

                // Bezier curve.
                let p0 = Vec2::new(start_distance, start_offset);
                let p2 = Vec2::new(end_distance, end_offset);

                let start_line = Line::new(p0.clone(), Vec2::new(1.0, start_angle));
                let end_line = Line::new(p2.clone(), Vec2::new(1.0, end_angle));

                let p1 = Geometry::line_intersect(&start_line, &end_line);
                
                let discr = 4.0 * (p1.x - p0.x) * (p1.x - p0.x) - 4.0 * (p0.x + p2.x - 2.0 * p1.x) * (p0.x - road_distance);
                let t0 = (2.0 * (p0.x - p1.x) + discr.sqrt()) / (2.0 * (p0.x + p2.x - 2.0 * p1.x));
                let t = if t0 > 0.0 && t0 < 1.0 { t0 } else { (2.0 * (p0.x - p1.x) - discr.sqrt()) / (2.0 * (p0.x + p2.x - 2.0 * p1.x)) };
                
                let offset = (1.0 - t) * (1.0 - t) * p0.y + 2.0 * t * (1.0 - t) * p1.y + t * t * p2.y;
                
                return offset;
            }
        }

        0.0
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