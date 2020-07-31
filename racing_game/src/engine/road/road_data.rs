use super::Math;

#[derive(Clone)]
pub struct CurvatureSegment {
    start : f32,
    end : f32,
    curvature : f32
}

impl CurvatureSegment {
    pub fn new(start : f32, end : f32, curvature : f32) -> CurvatureSegment {
        CurvatureSegment { start, end, curvature }
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
    start_offset : f32,
    curvatures : Vec<CurvatureSegment>,
    heels : Vec<Heel>
}

impl RoadData {
    pub fn new(start_offset : f32, length : f32, curvatures : Vec<CurvatureSegment>, heels : Vec<Heel>) -> RoadData {
        RoadData { track_length : length, start_offset, curvatures, heels }
    }

    pub fn get_norm_segment_offset(&self, prev_segment_offset : f32, curr_segment_start : f32) -> f32 {
        let seg_start_norm = curr_segment_start - self.start_offset;

        let mut curr_curvature = 0.0;

        for curvature_seg in &self.curvatures {
            if seg_start_norm > curvature_seg.start && seg_start_norm < curvature_seg.end { 
                curr_curvature = curvature_seg.curvature;
                break;
            };
        }

        curr_curvature + prev_segment_offset
    }

    pub fn get_hill_width_multiplier_delta(&self, vis_road_dist : f32) -> f32 {
        let vis_road_dist_norm = vis_road_dist - self.start_offset;

        for heel in &self.heels {
            if vis_road_dist_norm > heel.start && vis_road_dist_norm < heel.end {
                let steepness_t = (vis_road_dist_norm - heel.start) / (heel.end - heel.start);
                return Math::lerp(heel.start_steepness, heel.end_steepness, steepness_t);
            }
        }
        
        0.0
    }

    pub fn is_visible(&self, vis_road_dist : f32) -> bool {
        vis_road_dist >= self.start_offset && vis_road_dist <= self.start_offset + self.track_length
    }

    pub fn get_camera_pitch_delta(&self, camera_road_dist : f32) -> f32 {
        return self.get_hill_width_multiplier_delta(camera_road_dist) * 50.0;
    }
}