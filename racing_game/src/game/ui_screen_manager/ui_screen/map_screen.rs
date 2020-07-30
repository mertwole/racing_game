use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::{UIPage, UIText, UIImage, Pivot};
use crate::game::{Game, InputEvent, EventType};

use super::UIScreen;

pub struct MapScreen{
    page : UIPage,
    map_center_pos : IVec2,
    accesible_city_ids : Vec<usize>,

    city_sprite : RgbaImage,
    road_line_color : Rgb<u8>,
    road_line_width : u32,
}

impl MapScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> MapScreen {
        let mut map_page = UIPage::new(resolution.clone(), Some(Rgb([100, 100, 100])));

        let city_sprite = Game::load_image_rgba("ending_city_selected.png");

        MapScreen { 
            page : map_page, 
            map_center_pos : IVec2::new(320, 180), 
            city_sprite, 
            road_line_color : Rgb([0, 255, 0]), 
            road_line_width : 4, 
            accesible_city_ids : Vec::new() 
        }
    }
}

impl UIScreen for MapScreen {
    fn init(&mut self, game : &Game) {
        self.accesible_city_ids = game.city_map.get_accesible_city_ids();
    }   

    fn update(&mut self, game : &Game) {

    }  

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIUp, EventType::Pressed) => {  }
                (InputEvent::UIDown, EventType::Pressed) => {  }
                (InputEvent::UILeft, EventType::Pressed) => {  }
                (InputEvent::UIRight, EventType::Pressed) => {  }
                _ => { }
            }
        }
    }   

    fn render(&self, game : &Game, buffer : &mut RgbImage) {
        self.page.draw(buffer);

        // Render map.
        let map_size = game.city_map.size.clone();
        let left_bottom = &self.map_center_pos - &(&map_size / 2); 

        for road in &game.city_map.roads { 
            let road_start = &game.city_map.cities[road.source_id].position + &left_bottom;
            let road_end = &game.city_map.cities[road.destination_id].position + &left_bottom;
            ImageOps::draw_line(buffer, &road_start, &road_end, &self.road_line_color, self.road_line_width); 
        }

        let half_city_sprite_size = &IVec2::new(self.city_sprite.width() as isize, self.city_sprite.height() as isize) / 2;
        for city in &game.city_map.cities { 
            ImageOps::overlay_rgba(buffer, &self.city_sprite, &(&(&city.position + &left_bottom) - &half_city_sprite_size)); 
        }
    }
}