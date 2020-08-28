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

#[derive(Copy, Clone)]
struct Volume(usize);

pub struct SettingsScreen{
    menu : UISelector<MenuEvents>,
    music_volume : UISelector<Volume>,
    sfx_volume : UISelector<Volume>,
    page : UIPage,
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
                pivot : Pivot::Center, 
                position : IVec2::new(-90, 40), 
                binding : Binding::Center 
            }, 
            MenuEvents::MusicVolume)
        );
        // Sfx Volume.
        menu_items.push(UISelectorItem::new(
            Box::from(UIText::new(font.clone(), String::from("SFX VOLUME"))), 
            ControlProperties { 
                pivot : Pivot::Center, 
                position : IVec2::new(-90, 20), 
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

        let pointer_offset = IVec2::new(-(pointer_image.width() as isize), 0);
        let menu = UISelector::new(menu_items, SelectionType::Vertical, pointer_image, pointer_offset, resolution.clone(), None);

        let mut page = UIPage::new(*resolution, Some(Rgb([0, 0, 0])));

        let scale_img = Rc::from(Game::load_image_rgba("ui/volume_scale.png"));
        let music_volume_scale = Box::from(UIImage::new(scale_img.clone()));
        let sfx_volume_scale = Box::from(UIImage::new(scale_img));
        page.add_control(music_volume_scale, &ControlProperties { pivot : Pivot::Center, binding : Binding::Center, position : IVec2::new(50, 40) });
        page.add_control(sfx_volume_scale, &ControlProperties { pivot : Pivot::Center, binding : Binding::Center, position : IVec2::new(50, 20) });
        
        let mut music_volume_steps : Vec<UISelectorItem<Volume>> = Vec::new();
        for i in 0..12 {
            music_volume_steps.push(UISelectorItem::new(
                Box::from(UIVoid::new()), 
                ControlProperties { 
                    pivot : Pivot::Center, 
                    position : IVec2::new(-9 + i * 10, 40), 
                    binding : Binding::Center 
                }, 
                Volume(i as usize))
            );
        }
        let pointer_image = Game::load_image_rgba("ui/volume_pointer.png");
        let pointer_offset = IVec2::new(0, -((pointer_image.height() / 2) as isize));
        let music_volume = UISelector::new(music_volume_steps, SelectionType::Horizontal, pointer_image, pointer_offset, *resolution, None);


        let mut sfx_volume_steps : Vec<UISelectorItem<Volume>> = Vec::new();
        for i in 0..12 {
            sfx_volume_steps.push(UISelectorItem::new(
                Box::from(UIVoid::new()), 
                ControlProperties { 
                    pivot : Pivot::Center, 
                    position : IVec2::new(-9 + i * 10, 20), 
                    binding : Binding::Center 
                }, 
                Volume(i as usize))
            );
        }
        let pointer_image = Game::load_image_rgba("ui/volume_pointer.png");
        let pointer_offset = IVec2::new(0, -((pointer_image.height() / 2) as isize));
        let sfx_volume = UISelector::new(sfx_volume_steps, SelectionType::Horizontal, pointer_image, pointer_offset, *resolution, None);

        SettingsScreen { menu, page, music_volume, sfx_volume, selected_menu_item : MenuEvents::MusicVolume }
    }

    fn change_music_volume(&mut self, delta : isize) {
        self.music_volume.select_next_in_direction(&IVec2::new(delta, 0));
    }

    fn change_sfx_volume(&mut self, delta : isize) {
        self.sfx_volume.select_next_in_direction(&IVec2::new(delta, 0));
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
        self.page.draw(buffer);
        self.music_volume.draw(buffer);
        self.sfx_volume.draw(buffer);
        self.menu.draw(buffer);
    }
}