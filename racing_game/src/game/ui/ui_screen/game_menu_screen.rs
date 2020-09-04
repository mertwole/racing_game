use std::rc::Rc;

use image::{RgbImage, RgbaImage, Rgb};

use crate::engine::common::{IVec2, ImageOps};
use crate::engine::ui::font::*;
use crate::engine::ui::*;
use crate::game::*;
use crate::game::ui::*;

use super::UIScreen;

#[derive(Clone)]
enum MenuEvents {
    Resume,
    Settings,
    Exit
}

pub struct GameMenuScreen{
    menu : UISelector<MenuEvents>
}

impl GameMenuScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> GameMenuScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let mut menu_items : Vec<UISelectorItem<MenuEvents>> = Vec::new();

        // Resume.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("RESUME"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(320, 200), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Resume)
        );
        // Settings.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("SETTINGS"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(320, 180), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Settings)
        );
        // Exit to menu.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("EXIT TO MENU"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(320, 160), 
                binding : Binding::LeftBottom 
            }, 
            MenuEvents::Exit)
        );

        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let menu = UISelector::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, resolution.clone(), None);

        GameMenuScreen { menu }
    }
}

impl UIScreen for GameMenuScreen {
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
                        MenuEvents::Resume => { return vec![UIEvent::ChangeScreen(Screen::Game), UIEvent::SetRidePaused(false)]; },
                        MenuEvents::Settings => { return vec![UIEvent::ChangeScreen(Screen::Settings)]; },
                        MenuEvents::Exit => { return vec![]; },
                    }
                }
                (InputEvent::UIMenu, EventType::Pressed) => { return vec![UIEvent::PreviousScreen, UIEvent::SetRidePaused(false)]; }
                _ => { }
            }
        }

        Vec::new()
    }

    fn render(&self, buffer : &mut RgbImage) {
        self.menu.draw(buffer);
    }
}