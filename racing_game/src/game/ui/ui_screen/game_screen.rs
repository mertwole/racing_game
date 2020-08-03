use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

pub struct GameScreen{
    page : UIPage
}

impl GameScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GameScreen {
        let mut game_page = UIPage::new(resolution.clone(), None);
        let text = UIText::new(font, String::from("ABF"));
        game_page.add_control(Box::from(text), &ControlProperties { pivot : Pivot::LeftBottom, position : IVec2::new(10, 10), binding : Binding::LeftBottom } );

        GameScreen { page : game_page }
    }
}

impl UIScreen for GameScreen {
    fn init(&mut self, game : &Game) {
        
    }   

    fn update(&mut self, delta_time : f32) -> Vec<UIEvent> {
        Vec::new()
    }  

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                _ => { }
            }
        }
    }   

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
    }
}