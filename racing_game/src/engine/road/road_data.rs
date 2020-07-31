use super::Math;

pub trait OffsetSegment {
    fn get_offset(&self, distance : f32) -> Option<f32>;
}

#[derive(Clone)]
pub struct CurvedSegment {
    start : f32,
    end : f32,
    start_offset : f32,
    end_offset : f32
}

impl CurvedSegment {
    pub fn new(start : f32, end : f32, start_offset : f32, end_offset : f32) -> CurvedSegment {
        CurvedSegment { start, end, start_offset, end_offset }
    }
}

impl OffsetSegment for CurvedSegment {
    fn get_offset(&self, distance : f32) -> Option<f32> {
        if distance < self.start || distance > self.end { return None; }

        let lerp = (distance - self.start) / (self.end - self.start);
        let lerp = lerp * lerp * (3.0 - 2.0 * lerp); // Smoothstep from glsl.
        return Some(Math::lerp(self.start_offset, self.end_offset, lerp));
    }
}

#[derive(Clone)]
pub struct StraightSegment{
    start : f32,
    end : f32,
    offset : f32
}

impl StraightSegment {
    pub fn new(start : f32, end : f32, offset : f32) -> StraightSegment {
        StraightSegment { start, end, offset }
    }
}

impl OffsetSegment for StraightSegment {
    fn get_offset(&self, distance : f32) -> Option<f32> {
        if distance < self.start || distance > self.end { return None; }
        
        return Some(self.offset);
    }
}

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
}

#[derive(Clone)]
pub struct RoadData {
    track_length : f32,
    start_distance : f32,
    curved_segments : Vec<CurvedSegment>,
    straight_segments : Vec<StraightSegment>,
    heels : Vec<Heel>
}

impl RoadData {
    pub fn new(start_distance : f32, length : f32, curved_segments : Vec<CurvedSegment>, straight_segments : Vec<StraightSegment>, heels : Vec<Heel>) -> RoadData {
        RoadData { track_length : length, start_distance, curved_segments, straight_segments, heels }
    }

    pub fn get_norm_segment_offset(&self, road_distance : f32) -> f32 {
        for curved_seg in &self.curved_segments {
            if let Some(offset) = curved_seg.get_offset(road_distance) {
                return offset;
            }
        }

        for straight_seg in &self.straight_segments {
            if let Some(offset) = straight_seg.get_offset(road_distance) {
                return offset;
            }
        }

        0.0
    }

    pub fn get_hill_width_multiplier_delta(&self, vis_road_dist : f32) -> f32 {
        let vis_road_dist_norm = vis_road_dist - self.start_distance;

        for heel in &self.heels {
            if vis_road_dist_norm > heel.start && vis_road_dist_norm < heel.end {
                let steepness_t = (vis_road_dist_norm - heel.start) / (heel.end - heel.start);
                return Math::lerp(heel.start_steepness, heel.end_steepness, steepness_t);
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