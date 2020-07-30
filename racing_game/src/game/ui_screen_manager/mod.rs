use std::collections::HashMap;
use std::rc::Rc;

use image::{RgbImage, Rgb};

use crate::engine::ui::font::*;
use crate::engine::common::IVec2;
use super::{Game, EventType, InputEvent};

mod ui_screen;
use ui_screen::*;

pub struct UIScreenManager{
    screens : HashMap<Screen, Box<dyn UIScreen>>,
    current_screen : Screen
}

#[derive(Hash, Eq, PartialEq)]
pub enum Screen{
    Map,
    GameUI
}

impl UIScreenManager {
    pub fn new(resolution : &IVec2) -> UIScreenManager {
        let mut screens = HashMap::<Screen, Box<dyn UIScreen>>::new();

        let font = Font::new(Game::load_image_rgba("font.png"), IVec2::new(12, 12), String::from("ABCDEFGHIJ"));
        let font = Rc::from(font);

        let map_screen = Box::from(MapScreen::new(resolution, font.clone()));

        screens.insert(Screen::Map, map_screen);

        UIScreenManager { screens, current_screen : Screen::Map }
    }

    pub fn is_game_visible(&self) -> bool {
        match self.current_screen {
            Screen::GameUI => { true }
            _ => { false }
        }
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        self.screens.get_mut(&self.current_screen).unwrap().process_input(input);
    }

    pub fn update(&mut self, game : &mut Game) {
        self.screens.get_mut(&self.current_screen).unwrap().update(game);
    }   

    pub fn go_to_screen(&mut self, screen : Screen) {
        self.current_screen = screen;
    }

    pub fn render(&self, game : &Game, buffer : &mut RgbImage) {
        self.screens.get(&self.current_screen).unwrap().render(game, buffer);
    }
}