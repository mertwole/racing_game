use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::ui::selector_menu::*;
use crate::game::*;
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone)]
enum MenuEvents {
    Refuel,
    Back
}

pub struct GasStationsScreen{
    menu : SelectorMenu<MenuEvents>,
    menu_item_selected : bool,
    player : Option<Player>
}

impl GasStationsScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GasStationsScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let gas_stations_label = UIText::new(font.clone(), String::from("REFUEL"));
        let next_label = UIText::new(font.clone(), String::from("BACK"));

        let refuel_item = MenuItem::new(
            Box::from(gas_stations_label), 
            ControlProperties { 
                pivot : Pivot::LeftTop, 
                position : IVec2::new(20, -20), 
                binding : Binding::LeftTop 
            }, 
            MenuEvents::Refuel
        );

        let back_item = MenuItem::new(
            Box::from(next_label), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(20, 20), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Back
        );

        let menu = SelectorMenu::new(vec![refuel_item, back_item], pointer_image, resolution.clone());

        GasStationsScreen { menu, menu_item_selected : false, player : None }
    }

    fn refuel(&mut self) {
        let player = self.player.as_mut().unwrap();
        player.money -= 1;
        player.gas_level = Percent(100.0);
    }
}

impl UIScreen for GasStationsScreen {
    fn init(&mut self, game : &Game) {
        self.player = Some(game.player.clone());
    }

    fn update(&mut self, delta_time : f32) -> Vec<UIEvent> {
        if self.menu_item_selected {
            self.menu_item_selected = false;
            let menu_event = self.menu.select_current();
            match menu_event {
                MenuEvents::Refuel => { 
                    self.refuel();
                    return vec![UIEvent::ChangePlayer(self.player.clone().unwrap())]; 
                },
                MenuEvents::Back => { return vec![UIEvent::ChangeScreen(Screen::Services)]; } 
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