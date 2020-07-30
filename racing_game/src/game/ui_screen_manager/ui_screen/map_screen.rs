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

    city_sprite : RgbaImage,
    road_line_color : Rgb<u8>,
    road_line_width : u32,

    test_pos : IVec2
}

impl MapScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> MapScreen {
        let mut map_page = UIPage::new(resolution.clone(), Some(Rgb([100, 100, 100])));

        let test_image = Game::load_image_rgba("ferrari.png");
        
        let test_image = Rc::from(test_image);

        let text = UIText::new(font.clone(), String::from("ABC"));
        let image = UIImage::new(test_image.clone());

        map_page.add_control(Box::from(text), Pivot::Center, IVec2::new(100, 100));
        map_page.add_control(Box::from(image), Pivot::Center, IVec2::new(0, 0));

        let city_sprite = Game::load_image_rgba("city_circle.png");

        MapScreen { page : map_page, map_center_pos : IVec2::new(320, 180), city_sprite, road_line_color : Rgb([0, 255, 0]), road_line_width : 4, test_pos : IVec2::new(300, 300) }
    }
}

impl UIScreen for MapScreen {
    fn update(&mut self, game : &Game) {

    }   

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIUp, EventType::Pressed) => { self.test_pos = &self.test_pos + &IVec2::new(0, 10); }
                (InputEvent::UIDown, EventType::Pressed) => { self.test_pos = &self.test_pos + &IVec2::new(0, -10); }
                (InputEvent::UILeft, EventType::Pressed) => { self.test_pos = &self.test_pos + &IVec2::new(-10, 0); }
                (InputEvent::UIRight, EventType::Pressed) => { self.test_pos = &self.test_pos + &IVec2::new(10, 0); }
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
            ImageOps::draw_line(buffer, &(&road.source_pos + &left_bottom), &(&road.destination_pos + &left_bottom), &self.road_line_color, self.road_line_width); 
        }

        let half_city_sprite_size = &IVec2::new(self.city_sprite.width() as isize, self.city_sprite.height() as isize) / 2;
        for city in &game.city_map.cities { 
            ImageOps::overlay_rgba(buffer, &self.city_sprite, &(&(&city.position + &left_bottom) - &half_city_sprite_size)); 
        }

        ImageOps::overlay_rgba(buffer, &self.city_sprite, &self.test_pos); 
    }
}