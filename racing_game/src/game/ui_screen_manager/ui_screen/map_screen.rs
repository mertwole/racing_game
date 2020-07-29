use std::rc::Rc;

use image::{RgbImage, Rgb};

use crate::engine::common::IVec2;
use crate::engine::ui::font::*;
use crate::engine::ui::{UIPage, UIText, UIImage, Pivot};
use crate::game::Game;

use super::UIScreen;

pub struct MapScreen{
    page : UIPage
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

        MapScreen { page : map_page }
    }
}

impl UIScreen for MapScreen {
    fn update(&mut self, game : &Game) {

    }   

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
    }
}