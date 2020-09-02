use std::rc::Rc;
use std::collections::HashMap;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::window::Key;
use crate::game::key_name::*;
use crate::game::{Game, InputEvent, EventType};
use crate::game::ui::{UIEvent, Screen};

use super::UIScreen;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum MenuEvents {
    SteerLeft,
    SteerRight,
    Gas,
    Brake,

    Back
}

pub struct ControlSettingsScreen{
    page : UIPage,
    menu : UISelector<MenuEvents>,
    binding_action : Option<MenuEvents>,
    font : Rc<Font>,
    game : Option<Rc<Game>>,
    refresh_control_names : bool
}

impl ControlSettingsScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> ControlSettingsScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let mut menu_items : Vec<UISelectorItem<MenuEvents>> = Vec::new();

        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("STEER LEFT"))),
            ControlProperties { pivot : Pivot::RightBottom, binding : Binding::Center, position : IVec2::new(-10, 40) },
            MenuEvents::SteerLeft
        ));

        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("STEER RIGHT"))),
            ControlProperties { pivot : Pivot::RightBottom, binding : Binding::Center, position : IVec2::new(-10, 20) },
            MenuEvents::SteerRight
        ));

        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("GAS"))),
            ControlProperties { pivot : Pivot::RightBottom, binding : Binding::Center, position : IVec2::new(-10, 0) },
            MenuEvents::Gas
        ));

        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("BRAKE"))),
            ControlProperties { pivot : Pivot::RightBottom, binding : Binding::Center, position : IVec2::new(-10, -20) },
            MenuEvents::Brake
        ));

        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("BACK"))),
            ControlProperties { pivot : Pivot::Center, binding : Binding::Center, position : IVec2::new(0, -100) },
            MenuEvents::Back
        ));

        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let menu = UISelector::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, resolution.clone(), None);

        let page = UIPage::new(*resolution, Some(Rgb([0, 0, 0])));

        ControlSettingsScreen { page, menu, binding_action : None, font, game : None, refresh_control_names : false }
    }

    fn refresh_control_names(&mut self) {
        self.page.clear_controls();
        let game = self.game.as_ref().unwrap();

        let actions = vec![InputEvent::CarLeft, InputEvent::CarRight, InputEvent::CarGas, InputEvent::CarBrake];
        let positions = vec![40, 20, 0, -20];
        for (action, position) in actions.into_iter().zip(positions.into_iter()) {
            let bound_key = game.input.get_action_key(action);
            let bound_key = if bound_key.is_none() { String::from("UNBOUND") } else { bound_key.unwrap().key_name().unwrap_or(String::from("UNKNOWN")).to_uppercase() };
            let label = Box::from(UIText::new(self.font.clone(), bound_key));
            self.page.add_control(label, &ControlProperties { pivot : Pivot::LeftBottom, binding : Binding::Center, position : IVec2::new(10, position) });
        }
    }
}

impl UIScreen for ControlSettingsScreen {
    fn init(&mut self, game : &Game) {
        self.game = unsafe { Some(Rc::from_raw(game as *const Game)) };

        self.refresh_control_names();
    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        if self.refresh_control_names {
            self.refresh_control_names = false;
            self.refresh_control_names();
        }
        
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { 
                    if self.binding_action.is_none() {
                        self.menu.select_next_in_direction(&IVec2::new(0, -1)); 
                        continue;
                    }
                }
                (InputEvent::UIUp, EventType::Pressed) => { 
                    if self.binding_action.is_none() {
                        self.menu.select_next_in_direction(&IVec2::new(0, 1)); 
                        continue;
                    }
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    if self.binding_action.is_none() {
                        let menu_event = self.menu.select_current();
                        match menu_event {
                            MenuEvents::Back => { return vec![UIEvent::ChangeScreen(Screen::Settings)]; } // TODO : go back
                            _ => { self.binding_action = Some(menu_event); }
                        }
                        continue;
                    }
                }
                (InputEvent::AnyKey(key), EventType::Pressed) => { 
                    if self.binding_action.is_none() { continue; }
                    let key_name = key.key_name();
                    if key_name.is_some() {
                        let action = match self.binding_action.unwrap() {
                            MenuEvents::SteerLeft => { InputEvent::CarLeft }
                            MenuEvents::SteerRight => { InputEvent::CarRight }
                            MenuEvents::Gas => { InputEvent::CarGas }
                            MenuEvents::Brake => { InputEvent::CarBrake }
                            MenuEvents::Back => { panic!(); }
                        };
                        self.binding_action = None;
                        self.refresh_control_names = true;
                        return vec![UIEvent::BindKey(action, *key)];
                    }
                }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
        self.menu.draw(buffer);
    }
}