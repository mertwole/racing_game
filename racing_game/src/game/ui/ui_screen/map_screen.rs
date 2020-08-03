use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, Vec2, ImageOps, Math};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::{Game, InputEvent, EventType, city_map::city::CityDescription};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

pub struct MapScreen{
    page : UIPage,
    map_center_pos : IVec2,
    map_size : IVec2,

    intermediate_city_sprite : Rc<RgbaImage>,
    ending_city_sprite : Rc<RgbaImage>,
    city_selection_sprite : Rc<RgbaImage>,

    city_marks : Vec<CityMark>,
    accesible_city_ids : Vec<usize>,
    road_marks : Vec<RoadMark>,

    curr_selected_city_id : usize,
    selection_mark_pos : IVec2,

    start_ride_flag : bool
}

struct CityMark {
    position : IVec2,
    ending : bool,
}

struct RoadMark {
    source : IVec2,
    destination : IVec2,
    accesible : bool
}

impl RoadMark {
    fn color(&self) -> Rgb<u8> {
        if self.accesible { Rgb([0, 0, 255]) } else { Rgb([0, 255, 0]) }
    }

    fn width(&self) -> u32 { 4 }
}

impl MapScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> MapScreen {
        let map_page = UIPage::new(resolution.clone(), None);

        let intermediate_city_sprite = Rc::from(Game::load_image_rgba("ui/intermediate_city.png"));
        let ending_city_sprite = Rc::from(Game::load_image_rgba("ui/ending_city.png"));
        let city_selection_sprite = Rc::from(Game::load_image_rgba("ui/city_selection.png"));

        MapScreen { 
            page : map_page, 

            map_center_pos : IVec2::new(320, 180), 
            map_size : IVec2::zero(),

            intermediate_city_sprite, 
            ending_city_sprite,
            city_selection_sprite,

            city_marks : Vec::new(),
            accesible_city_ids : Vec::new(),
            road_marks : Vec::new(),

            curr_selected_city_id : 0,
            selection_mark_pos : IVec2::zero(),

            start_ride_flag : false,
        }
    }

    fn change_selected_city(&mut self, direction : &IVec2) {
        let mut min_dist_in_direction = 100000;
        let selected_city_pos = self.city_marks[self.curr_selected_city_id].position;
        for &city_id in &self.accesible_city_ids {
            let city_pos = self.city_marks[city_id].position;
            let dist_in_direction = direction.dot(&(&city_pos - &selected_city_pos));
            if dist_in_direction > 0 && dist_in_direction < min_dist_in_direction {
                min_dist_in_direction = dist_in_direction;
                self.curr_selected_city_id = city_id;
            }
        }
    }
}

impl UIScreen for MapScreen {
    fn init(&mut self, game : &Game) {
        self.accesible_city_ids = game.city_map.get_accesible_city_ids();

        self.map_size = game.city_map.size;

        self.city_marks = game.city_map.cities.iter()
        .map(|city| CityMark { 
            position : city.position, 
            ending : match city.description {
                CityDescription::Intermediate => { false },
                _ => { true }
            }
        })
        .collect();

        self.curr_selected_city_id = self.accesible_city_ids[0];
        self.selection_mark_pos = self.city_marks[self.accesible_city_ids[0]].position;

        self.accesible_city_ids = game.city_map.get_accesible_city_ids();

        let accessible_road_ids = game.city_map.get_accesible_road_ids();

        self.road_marks = game.city_map.roads.iter()
        .enumerate()
        .map(|(id, road)| RoadMark { 
            source : self.city_marks[road.source_id].position,
            destination : self.city_marks[road.destination_id].position,
            accesible : accessible_road_ids.contains(&id)
        })
        .collect();

        let map_left_bottom = &self.map_center_pos - &(&self.map_size / 2);

        self.page.clear_controls();

        for city_mark in &self.city_marks {
            let image = UIImage::new(if city_mark.ending { self.ending_city_sprite.clone() } else { self.intermediate_city_sprite.clone() });
            self.page.add_control(Box::from(image), &ControlProperties { pivot : Pivot::Center, position : &city_mark.position + &map_left_bottom, binding : Binding::LeftBottom });
        }
    }   

    fn update(&mut self, delta_time : f32) -> Vec<UIEvent>{
        if self.start_ride_flag {
            self.start_ride_flag = false;
            return vec![
                UIEvent::SelectCityDestination(self.curr_selected_city_id),
                UIEvent::StartRide, 
                UIEvent::ChangeScreen(Screen::Game)
            ];
        }

        // Interpolate selection mark.
        let dest_selection_mark_pos = self.city_marks[self.curr_selected_city_id].position;
        let new_selection_mark_pos = Vec2::new(
            Math::lerp(self.selection_mark_pos.x as f32, dest_selection_mark_pos.x as f32, 0.5),
            Math::lerp(self.selection_mark_pos.y as f32, dest_selection_mark_pos.y as f32, 0.5)
        );
        self.selection_mark_pos = IVec2::new(
            if (dest_selection_mark_pos.x - self.selection_mark_pos.x) > 0 { new_selection_mark_pos.x.ceil() } else { new_selection_mark_pos.x.floor() } as isize,
            if (dest_selection_mark_pos.y - self.selection_mark_pos.y) > 0 { new_selection_mark_pos.y.ceil() } else { new_selection_mark_pos.y.floor() } as isize
        );

        Vec::new()
    }  

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIUp, EventType::Pressed) => { self.change_selected_city(&IVec2::new(0, 1)); }
                (InputEvent::UIDown, EventType::Pressed) => { self.change_selected_city(&IVec2::new(0, -1)); }
                (InputEvent::UILeft, EventType::Pressed) => { self.change_selected_city(&IVec2::new(-1, 0)); }
                (InputEvent::UIRight, EventType::Pressed) => { self.change_selected_city(&IVec2::new(1, 0)); }

                (InputEvent::UISelect, EventType::Pressed) => { self.start_ride_flag = true; }
                _ => { }
            }
        }
    }   

    fn render(&self, buffer : &mut RgbImage) {
        ImageOps::fill_with_color(buffer, &Rgb([100, 100, 100]));
        // Render roads.
        let map_left_bottom = &self.map_center_pos - &(&self.map_size / 2); 

        for road in &self.road_marks { 
            let road_start = &road.source + &map_left_bottom;
            let road_end = &road.destination + &map_left_bottom;
            ImageOps::draw_line(buffer, &road_start, &road_end, &road.color(), road.width()); 
        }

        self.page.draw(buffer);

        let selection_mark_half_size = &IVec2::new(
            self.city_selection_sprite.as_ref().width() as isize, 
            self.city_selection_sprite.as_ref().height() as isize) / 2;
        let selection_mark_pos = &(&map_left_bottom + &self.selection_mark_pos) - &selection_mark_half_size;
        ImageOps::overlay_rgba(buffer, self.city_selection_sprite.as_ref(), &selection_mark_pos);
    }
}