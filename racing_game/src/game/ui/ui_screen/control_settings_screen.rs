use std::rc::Rc;
use std::collections::HashMap;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::engine::window::Key;
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
    key_binding_control_ids : HashMap<MenuEvents, usize>,
    binding_action : Option<MenuEvents>
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

        let mut page = UIPage::new(*resolution, Some(Rgb([0, 0, 0])));
        page.add_control(Box::from(menu), &ControlProperties { pivot : Pivot::Center, binding : Binding::Center, position : IVec2::new(0, 0) });

        let mut key_binding_control_ids = HashMap::new();
        key_binding_control_ids.insert(MenuEvents::SteerLeft, 0);
        key_binding_control_ids.insert(MenuEvents::SteerRight, 1);
        key_binding_control_ids.insert(MenuEvents::Gas, 2);
        key_binding_control_ids.insert(MenuEvents::Brake, 3);

        let steer_left_label = Box::from(UIText::new(font.clone(), String::from("LEFT ARROW")));
        page.add_control(steer_left_label, &ControlProperties { pivot : Pivot::LeftBottom, binding : Binding::Center, position : IVec2::new(10, 40) });

        let steer_right_label = Box::from(UIText::new(font.clone(), String::from("RIGHT ARROW")));
        page.add_control(steer_right_label, &ControlProperties { pivot : Pivot::LeftBottom, binding : Binding::Center, position : IVec2::new(10, 20) });

        let gas_label = Box::from(UIText::new(font.clone(), String::from("UP ARROW")));
        page.add_control(gas_label, &ControlProperties { pivot : Pivot::LeftBottom, binding : Binding::Center, position : IVec2::new(10, 0) });

        let brake_label = Box::from(UIText::new(font.clone(), String::from("DOWN ARROW")));
        page.add_control(brake_label, &ControlProperties { pivot : Pivot::LeftBottom, binding : Binding::Center, position : IVec2::new(10, -20) });

        ControlSettingsScreen { page, key_binding_control_ids, binding_action : None }
    }

    fn get_bound_key_name(&self, action : MenuEvents) -> String {
        String::from("")
    }
}

impl UIScreen for ControlSettingsScreen {
    fn init(&mut self, game : &Game) {
        
    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        let menu = unsafe { &mut *(self.page.get_control_mut(0) as *mut dyn UIControl as *mut UISelector<MenuEvents>) };

        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { menu.select_next_in_direction(&IVec2::new(0, -1)); }
                (InputEvent::UIUp, EventType::Pressed) => { menu.select_next_in_direction(&IVec2::new(0, 1)); }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    let menu_event = menu.select_current();
                    match menu_event {
                        MenuEvents::Back => { return vec![UIEvent::ChangeScreen(Screen::Settings)]; } // TODO : go back
                        _ => { self.binding_action = Some(menu_event); }
                    }
                }
                (InputEvent::AnyKey(key), EventType::Pressed) => { println!("{}", key.get_name().unwrap_or(String::from("unexpected"))); }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.page.draw(buffer);
    }
}