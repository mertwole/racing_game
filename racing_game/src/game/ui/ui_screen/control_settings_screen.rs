use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone)]
enum MenuEvents {

}

pub struct ControlSettingsScreen{
    menu : UISelector<MenuEvents>
}

impl ControlSettingsScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ControlSettingsScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let mut menu_items : Vec<UISelectorItem<MenuEvents>> = Vec::new();

        
        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let menu = UISelector::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, resolution.clone(), None);

        ControlSettingsScreen { menu }
    }
}

impl UIScreen for ControlSettingsScreen {
    fn init(&mut self, game : &Game) {

    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, -1)); }
                (InputEvent::UIUp, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, 1)); }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    let menu_event = self.menu.select_current();
                    match menu_event {
                        
                    }
                }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.draw(buffer);
    }
}