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
    Hostels,
    RepairStations,
    Shops,

    Next
}

pub struct ServicesScreen{
    menu : SelectorMenu<MenuEvents>
}

impl ServicesScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ServicesScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let mut menu_items : Vec<MenuItem<MenuEvents>> = Vec::new();

        // Gas stations.
        menu_items.push(MenuItem::new(
            Box::from(UIText::new(font.clone(), String::from("GAS STATION"))), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -20), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::GasStations)
        );
        // Hostels.
        menu_items.push(MenuItem::new(
            Box::from(UIText::new(font.clone(), String::from("HOSTEL"))), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -40), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::Hostels)
        );
        // Repair stations.
        menu_items.push(MenuItem::new(
            Box::from(UIText::new(font.clone(), String::from("REPAIR STATION"))), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -60), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::RepairStations)
        );
        // Shops.
        menu_items.push(MenuItem::new(
            Box::from(UIText::new(font.clone(), String::from("SHOP"))), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -80), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::Shops)
        );
        // Next.
        menu_items.push(MenuItem::new(
            Box::from(UIText::new(font.clone(), String::from("NEXT"))), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(20, 20), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Next)
        );

        let menu = SelectorMenu::new(menu_items, pointer_image, resolution.clone());

        ServicesScreen { menu }
    }
}

impl UIScreen for ServicesScreen {
    fn init(&mut self, game : &Game) {

    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, 1)); }
                (InputEvent::UIUp, EventType::Pressed) => { self.menu.select_next_in_direction(&IVec2::new(0, -1)); }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    let menu_event = self.menu.select_current();
                    match menu_event {
                        MenuEvents::GasStations => { return vec![UIEvent::ChangeScreen(Screen::GasStations)]; },
                        MenuEvents::Hostels => { return vec![UIEvent::ChangeScreen(Screen::Hostels)]; },
                        MenuEvents::RepairStations => { return vec![UIEvent::ChangeScreen(Screen::RepairStations)]; },
                        MenuEvents::Shops => { return vec![UIEvent::ChangeScreen(Screen::Shops)]; },

                        MenuEvents::Next => { return vec![UIEvent::ChangeScreen(Screen::Map)]; } 
                    }
                }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.render(buffer);
    }
}