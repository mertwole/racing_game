use rand::{RngCore, rngs::StdRng};

use crate::engine::math::{IVec2, LineSegment, Geometry};

mod city;
mod road;

use city::*;
use road::*;

pub struct GenerationParameters{
    pub city_count : u32,
    pub grid_size : IVec2,
    pub min_distance_between_cities : f32
}

pub struct CityMap{
    cities : Vec<City>
}

impl CityMap {
    pub fn generate(rng : &mut StdRng, parameters : GenerationParameters) -> CityMap {
        let mut city_positions : Vec<IVec2> = Vec::new();

        // Fill city positions.
        let min_city_dist_sqr = parameters.min_distance_between_cities * parameters.min_distance_between_cities;
        for _i in 0..parameters.city_count {
            // Regenerate city while it is too close to another cities.
            'outer : loop {  
                let new_city_pos = IVec2::new(rng.next_u32() as isize % parameters.grid_size.x, rng.next_u32() as isize % parameters.grid_size.x);
                
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

        // First, all possible city pairs.
        let mut roads : Vec<(usize, usize)> = Vec::with_capacity((city_positions.len() * (city_positions.len())) / 2);
        for i in 0..city_positions.len() {
            for j in i + 1..city_positions.len() {
                roads.push((i, j)); 
            }
        }

        // Remove all intersecting roads (remove longer).
        let (mut i, mut j) = (0, 0);
        loop {
            let meeting_at_one_city = roads[i].0 == roads[j].0 || roads[i].1 == roads[j].0 || roads[i].0 == roads[j].1 || roads[i].1 == roads[j].1;

            // So they can intersect.
            if !meeting_at_one_city {
                let road_i_segment = LineSegment::new(city_positions[roads[i].0].vec2(), city_positions[roads[i].1].vec2());
                let road_j_segment = LineSegment::new(city_positions[roads[j].0].vec2(), city_positions[roads[j].1].vec2());
                
                if !meeting_at_one_city && Geometry::line_segment_intersect(&road_i_segment, &road_j_segment) {
                    let road_i_sqr_len = road_i_segment.sqr_length();
                    let road_j_sqr_len = road_j_segment.sqr_length();

                    if road_i_sqr_len > road_j_sqr_len {
                        roads.remove(i);
                        i -= 1;
                        j -= 1;
                    } else {
                        roads.remove(j);
                        j -= 1;
                    }
                }
            }

            j += 1;
            if j >= roads.len() { 
                i += 1; 
                j = i + 1; 
                if i >= roads.len() - 1 { break; }
            }
        }

        for road in roads { println!("source : {} destination : {}", road.0, road.1); }

        let cities : Vec<City> = city_positions.into_iter().map(|pos| City { position : pos }).collect();

        for y in 0..parameters.grid_size.y {
            'x : for x in 0..parameters.grid_size.x {
                for i in 0..cities.len() {
                    if cities[i].position.x == x && cities[i].position.y == y { print!("{}", i); continue 'x; }
                }
                print!("#");
            }
            println!();
        }

        println!("generated!");

        CityMap { cities }
    }
}