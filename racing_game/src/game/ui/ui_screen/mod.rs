use std::collections::HashMap;
use std::rc::Rc;

use image::{RgbImage};

use crate::engine::common::IVec2;
use crate::game::{Game, InputEvent, EventType};
use super::UIEvent;
use crate::engine::ui::font::*;

mod map_screen;
mod game_screen;
mod services_screen;
mod gas_stations_screen;

pub use map_screen::*;
pub use game_screen::*;
pub use services_screen::*;
pub use gas_stations_screen::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Screen{
    Map,
    Game,
    Services,
    GasStations
}

pub fn create_all_screens(resolution : &IVec2) -> HashMap<Screen, Box<dyn UIScreen>>{
    let mut ui_screens = HashMap::<Screen, Box<dyn UIScreen>>::new();

    let font = Font::new(Game::load_image_rgba("font.png"), IVec2::new(12, 12), String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890$% "));
    let font = Rc::from(font);

    let map_screen = Box::from(MapScreen::new(resolution, font.clone()));
    let game_screen = Box::from(GameScreen::new(resolution, font.clone()));
    let services_screen = Box::from(ServicesScreen::new(resolution, font.clone()));
    let gas_stations_screen = Box::from(GasStationsScreen::new(resolution, font.clone()));

    ui_screens.insert(Screen::Map, map_screen);
    ui_screens.insert(Screen::Game, game_screen);
    ui_screens.insert(Screen::Services, services_screen);
    ui_screens.insert(Screen::GasStations, gas_stations_screen);

    ui_screens
}

pub trait UIScreen {
    fn update(&mut self, delta_time : f32) -> Vec<UIEvent>;
    fn init(&mut self, game : &Game);
    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>);
    fn render(&self, buffer : &mut RgbImage);
}