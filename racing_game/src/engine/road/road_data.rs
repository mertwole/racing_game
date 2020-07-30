use super::Math;

struct CurvatureSegment {
    start : f32,
    end : f32,
    curvature : f32
}

struct Heel{
    start : f32,
    end : f32,
    start_steepness : f32,
    end_steepness : f32
}

pub struct RoadData {
    track_length : f32,
    start_offset : f32,
    curvatures : Vec<CurvatureSegment>,
    heels : Vec<Heel>
}

impl RoadData {
    pub fn new() -> RoadData {
        let curvatures = vec![
            CurvatureSegment { start : 10.0, end : 30.0, curvature : 0.00005},
            CurvatureSegment { start : 20.0, end : 40.0, curvature : -0.00001}
        ];

        let heels = vec![
            Heel { start : 0.0, end : 25.0, start_steepness : 0.0, end_steepness : 0.001 },
            Heel { start : 25.0, end : 75.0, start_steepness : 0.001, end_steepness : -0.001 },
            Heel { start : 75.0, end : 100.0, start_steepness : -0.001, end_steepness : 0.0 }
        ];

        RoadData { track_length : 150.0, start_offset : 20.0, curvatures, heels }
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