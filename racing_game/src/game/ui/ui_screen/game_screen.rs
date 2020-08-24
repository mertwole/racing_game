use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

pub struct GameScreen{
    page : UIPage,
    game : Option<Rc<Game>>,
    font : Rc<Font>
}

impl GameScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GameScreen {
        GameScreen { page : UIPage::new(resolution.clone(), None), game : None, font }
    }
}

impl UIScreen for GameScreen {
    fn init(&mut self, game : &Game) {
        self.game = unsafe { Some(Rc::from_raw(game as *const Game)) };
    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        self.page.clear_controls();

        let speed = self.game.as_ref().unwrap().ride.car.speed;
        let speed_label = UIText::new(self.font.clone(), String::from(format!("{} MPH", speed)));
        
        self.page.add_control(Box::from(speed_label), &ControlProperties { position : IVec2::new(10, -10), pivot : Pivot::LeftTop, binding : Binding::LeftTop });

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIMenu, EventType::Pressed) => { return vec![UIEvent::ChangeScreen(Screen::GameMenu), UIEvent::SetRidePaused(true)]; }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
    }
}