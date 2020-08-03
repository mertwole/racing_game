use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::{UIPage, UIText, UIImage, Pivot};
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

pub struct ServicesScreen{
    page : UIPage
}

impl ServicesScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ServicesScreen {
        let mut page = UIPage::new(resolution.clone(), Some(Rgb([100, 100, 100])));
        let text = UIText::new(font, String::from("ABF"));
        page.add_control(Box::from(text), Pivot::LeftBottom, IVec2::new(10, 10));

        ServicesScreen { page }
    }
}

impl UIScreen for ServicesScreen {
    fn init(&mut self, game : &Game) {
        let services = game.city_map.get_current_city_services();

        self.page.clear_controls();

        let logo_img = Box::from(UIImage::new(services.gas_stations[0].logo.clone()));
        self.page.add_control(logo_img, Pivot::Center, IVec2::new(300, 200));
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