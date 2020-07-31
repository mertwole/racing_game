use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::{UIPage, UIText, UIImage, Pivot};
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

pub struct GameScreen{
    page : UIPage,
    arrived_to_city_flag : bool
}

impl GameScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GameScreen {
        let mut game_page = UIPage::new(resolution.clone(), None);
        let text = UIText::new(font, String::from("ABF"));
        game_page.add_control(Box::from(text), Pivot::LeftBottom, IVec2::new(10, 10));

        GameScreen { page : game_page, arrived_to_city_flag : false }
    }
}

impl UIScreen for GameScreen {
    fn init(&mut self, game : &Game) {
        
    }   

    fn update(&mut self, delta_time : f32) -> Vec<UIEvent> {
        if self.arrived_to_city_flag {
            self.arrived_to_city_flag = false;
            return vec![UIEvent::RideFinished, UIEvent::ChangeScreen(Screen::Map)];
        }

        Vec::new()
    }  

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UISelect, EventType::Pressed) => { self.arrived_to_city_flag = true; }
                _ => { }
            }
        }
    }   

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
    }
}