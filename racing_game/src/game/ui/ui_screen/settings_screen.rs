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
    MusicVolume,
    SfxVolume,
    ControlSettings,

    Back
}

pub struct SettingsScreen{
    menu : UISelector<MenuEvents>,
    selected_menu_item : MenuEvents
}

impl SettingsScreen {
    pub fn new(resolution : &IVec2, font : Rc<Font>) -> SettingsScreen {
        let pointer_image = Game::load_image_rgba("ui/pointer.png");

        let mut menu_items : Vec<UISelectorItem<MenuEvents>> = Vec::new();

        // Music volume.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("MUSIC VOLUME"))), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(-50, 40), 
                binding : Binding::Center 
            }, 
            MenuEvents::MusicVolume)
        );
        // Sfx Volume.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("SFX VOLUME"))), 
            ControlProperties { 
                pivot : Pivot::LeftBottom, 
                position : IVec2::new(-50, 20), 
                binding : Binding::Center 
            }, 
            MenuEvents::SfxVolume)
        );
        // Control settings.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("CONTROL SETTINGS"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(0, 0), 
                binding : Binding::Center 
            }, 
            MenuEvents::ControlSettings)
        );
        // Back.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("BACK"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(0, -140), 
                binding : Binding::Center 
            }, 
            MenuEvents::Back)
        );

        let menu = UISelector::new(menu_items, SelectionType::Vertical, pointer_image, resolution.clone(), Some(Rgb([0, 0, 0])));

        SettingsScreen { menu, selected_menu_item : MenuEvents::MusicVolume }
    }

    fn change_music_volume(&mut self, delta : isize) {

    }

    fn change_sfx_volume(&mut self, delta : isize) {

    }
}

impl UIScreen for SettingsScreen {
    fn init(&mut self, game : &Game) {

    }   

    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        for (event, event_type) in input {
            match (event, event_type) {
                (InputEvent::UIDown, EventType::Pressed) => { 
                    self.menu.select_next_in_direction(&IVec2::new(0, -1)); 
                    self.selected_menu_item = self.menu.select_current();
                }
                (InputEvent::UIUp, EventType::Pressed) => { 
                    self.menu.select_next_in_direction(&IVec2::new(0, 1)); 
                    self.selected_menu_item = self.menu.select_current();
                }
                (InputEvent::UILeft, EventType::Pressed) => {
                    match self.selected_menu_item {
                        MenuEvents::MusicVolume => { self.change_music_volume(-1); }
                        MenuEvents::SfxVolume => { self.change_sfx_volume(-1); }
                        _ => { }
                    }
                }
                (InputEvent::UIRight, EventType::Pressed) => {
                    match self.selected_menu_item {
                        MenuEvents::MusicVolume => { self.change_music_volume(1); }
                        MenuEvents::SfxVolume => { self.change_sfx_volume(1); }
                        _ => { }
                    }
                }
                (InputEvent::UISelect, EventType::Pressed) => { 
                    match self.selected_menu_item {
                        MenuEvents::ControlSettings => { return vec![UIEvent::ChangeScreen(Screen::ControlSettings)]; },
                        MenuEvents::Back => { return vec![UIEvent::ChangeScreen(Screen::GameMenu)]; },// TODO : add UndoScreen command. 

                        MenuEvents::MusicVolume => { return vec![]; },
                        MenuEvents::SfxVolume => { return vec![]; }
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