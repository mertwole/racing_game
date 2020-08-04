use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::ui::selector_menu::*;
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone)]
enum MenuEvents {
    GasStations,
    Next
}

pub struct ServicesScreen{
    menu : SelectorMenu<MenuEvents>,
    menu_item_selected : bool
}

impl ServicesScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ServicesScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let gas_stations_label = UIText::new(font.clone(), String::from("GAS STATIONS"));
        let next_label = UIText::new(font.clone(), String::from("NEXT"));

        let gas_stations_item = MenuItem::new(
            Box::from(gas_stations_label), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -20), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::GasStations
        );

        let next_item = MenuItem::new(
            Box::from(next_label), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(20, 20), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Next
        );

        let menu = SelectorMenu::new(vec![gas_stations_item, next_item], pointer_image, resolution.clone());

        ServicesScreen { menu, menu_item_selected : false }
    }
}

impl UIScreen for ServicesScreen {
    fn init(&mut self, game : &Game) {

    }   

    fn update(&mut self, delta_time : f32) -> Vec<UIEvent> {
        if self.menu_item_selected {
            self.menu_item_selected = false;
            let menu_event = self.menu.select_current();
            match menu_event {
                MenuEvents::GasStations => { return vec![]; },
                MenuEvents::Next => { return vec![UIEvent::ChangeScreen(Screen::Map)]; } 
            }
        }

        Vec::new()
    }  

    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, -1)); }
                (InputEvent::UIUp, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, 1)); }
                (InputEvent::UISelect, EventType::Pressed) => { self.menu_item_selected = true; }
                _ => { }
            }
        }
    }   

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.render(buffer);
    }
}