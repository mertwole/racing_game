use rand::{RngCore, rngs::StdRng};

use crate::engine::common::{IVec2, LineSegment, Geometry};

pub mod city;
pub mod road_path;

use city::*;
use road_path::*;

pub struct GenerationParameters{
    pub city_count : u32,
    pub size : IVec2,
    pub min_distance_between_cities : f32
}

#[readonly::make]
pub struct CityMap{
    pub cities : Vec<City>,
    pub roads : Vec<RoadPath>,
    pub size : IVec2,
    pub start_city_id : usize,
    pub finish_city_id : usize,
    pub current_city_id : usize
}

impl CityMap {
    // Checks graph that conatins verts [0..verts_count);
    fn check_graph_coherency(verts_count : usize, connections : &Vec<(usize, usize)>, source : usize, destination : usize) -> bool {       
        #[derive(Clone)]
        enum VertState {
            UNCHECKED,
            CHECKED,
            TO_CHECK,
            DESTINATION
        }

        let mut vert_states : Vec<VertState> = vec![VertState::UNCHECKED; verts_count];
        vert_states[source] = VertState::TO_CHECK;
        vert_states[destination] = VertState::DESTINATION;
        
        loop {
            let mut to_check_exists = false;
            for i in 0..vert_states.len() {
                match vert_states[i] {
                    VertState::TO_CHECK => { 
                        to_check_exists = true;

                        for connection in connections {
                            if connection.0 == i {
                                match vert_states[connection.1] {
                                    VertState::DESTINATION => { return true; }
                                    VertState::UNCHECKED => { vert_states[connection.1] = VertState::TO_CHECK }
                                    _ => { }
                                }    
                            } else if connection.1 == i {
                                match vert_states[connection.0] {
                                    VertState::DESTINATION => { return true; }
                                    VertState::UNCHECKED => { vert_states[connection.0] = VertState::TO_CHECK }
                                    _ => { }
                                } 
                            }
                        }

                        vert_states[i] = VertState::CHECKED;
                    }
                    _ => {}
                }
            }

            if !to_check_exists { return false; } 
        }
         
    }

    pub fn generate(rng : &mut StdRng, parameters : GenerationParameters) -> CityMap {
        // 1. Generate random points in grid.
        // 2. Connect all the points.
        // 3. Remove intersecting(longest of the pair).
        // 4. Remove random roads keeping graph coherency(it's better to remove roads among longest).
        let mut city_positions : Vec<IVec2> = Vec::new();

        // Fill city positions.
        let min_city_dist_sqr = parameters.min_distance_between_cities * parameters.min_distance_between_cities;
        for _i in 0..parameters.city_count {
            // Regenerate city while it is too close to another cities.
            'outer : loop {  
                let new_city_pos = IVec2::new(rng.next_u32() as isize % parameters.size.x, rng.next_u32() as isize % parameters.size.x);
                
                for city_position in &city_positions {
                    let new_city_dist_sqr = (&new_city_pos - &city_position).sqr_len() as f32;
                    if new_city_dist_sqr < min_city_dist_sqr {
                        continue 'outer;
                    }
                }

                city_positions.push(new_city_pos);
                break;
            }   
        }

        // Select start and end cities.
        let mut start_city_id = 0; // Left bottom city.
        let mut finish_city_id = 0; // Right top city.
        for i in 0..city_positions.len() {
            if city_positions[i].x < city_positions[start_city_id].x 
            || (city_positions[i].x == city_positions[start_city_id].x && city_positions[i].y < city_positions[start_city_id].y) {
                start_city_id = i;
            }

            if city_positions[i].x > city_positions[finish_city_id].x 
            || (city_positions[i].x == city_positions[finish_city_id].x && city_positions[i].y > city_positions[finish_city_id].y) {
                finish_city_id = i;
            }
        }   

        // First, all possible city pairs.
        let mut roads : Vec<(usize, usize)> = Vec::with_capacity((city_positions.len() * (city_positions.len())) / 2);
        for i in 0..city_positions.len() {
            for j in i + 1..city_positions.len() {
                roads.push((i, j)); 
            }
        }

        // Sort by length.
        roads.sort_by(|a, b| 
            (&city_positions[a.0] - &city_positions[a.1]).sqr_len()
            .cmp(&(&city_positions[b.0] - &city_positions[b.1]).sqr_len())
        );

        // Remove all intersecting roads (remove longest, roads are sorted by length => remove with largest index => remove j'th ).
        let (mut i, mut j) = (0, 0);
        loop {
            let meeting_at_one_city = roads[i].0 == roads[j].0 || roads[i].1 == roads[j].0 || roads[i].0 == roads[j].1 || roads[i].1 == roads[j].1;

            // So they can potentially intersect.
            if !meeting_at_one_city {
                let road_i_segment = LineSegment::new(city_positions[roads[i].0].vec2(), city_positions[roads[i].1].vec2());
                let road_j_segment = LineSegment::new(city_positions[roads[j].0].vec2(), city_positions[roads[j].1].vec2());
                
                if !meeting_at_one_city && Geometry::line_segment_intersect(&road_i_segment, &road_j_segment) {
                    roads.remove(j);
                    j -= 1;
                }
            }

            j += 1;
            if j >= roads.len() { 
                i += 1; 
                j = i + 1; 
                if i >= roads.len() - 1 { break; }
            }
        }

        // Remove [remove_count] random roads among [removable_road_count] longest.
        let removable_road_count = roads.len() / 2;
        let unremovable_road_count = roads.len() - removable_road_count;
        let remove_count = roads.len() / 3;
        
        let mut removed = 0;
        for _i in 0..remove_count {
            let remove_id = unremovable_road_count + rng.next_u32() as usize % (removable_road_count - removed);
            let removed_road = roads.remove(remove_id);
            if !CityMap::check_graph_coherency(city_positions.len(), &roads, removed_road.0, removed_road.1) {
                roads.insert(remove_id, removed_road);
            } else { removed += 1; }
        }     

        let roads : Vec<RoadPath> = roads.into_iter().map(|road| RoadPath::new(road.0, road.1)).collect();
        let cities : Vec<City> = city_positions.into_iter().map(|pos| City { position : pos }).collect();

        CityMap { cities, roads, size : parameters.size, start_city_id, finish_city_id, current_city_id : start_city_id }
    }

    pub fn get_accesible_city_ids(&self) -> Vec<usize>{
        let mut accesible : Vec<usize> = Vec::new();
        for road in &self.roads {
            if road.source_id == self.current_city_id {
                accesible.push(road.destination_id);
            }

            if road.destination_id == self.current_city_id {
                accesible.push(road.source_id);
            }
        }

        accesible
    }
}