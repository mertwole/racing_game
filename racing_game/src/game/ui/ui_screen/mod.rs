use std::collections::HashMap;
use std::rc::Rc;

use image::{RgbImage};

use crate::engine::common::IVec2;
use crate::game::{Game, InputEvent, EventType};
use crate::game::city_map::services::*;
use super::UIEvent;
use crate::engine::ui::font::*;

mod map_screen;
mod game_screen;
mod game_menu_screen;
mod services_screen;
mod service_select_screen;
mod settings_screen;
mod control_settings_screen;

pub use map_screen::*;
pub use game_screen::*;
pub use game_menu_screen::*;
pub use services_screen::*;
pub use service_select_screen::*;
pub use settings_screen::*;
pub use control_settings_screen::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Screen{
    Map,
    Game,
    GameMenu,
    Services,
    Settings,
    ControlSettings,

    GasStations,
    Hostels,
    RepairStations,
    Shops
}

pub fn create_all_screens(resolution : &IVec2) -> HashMap<Screen, Box<dyn UIScreen>>{
    let mut ui_screens = HashMap::<Screen, Box<dyn UIScreen>>::new();

    let font = Font::new(Game::load_image_rgba("font.png"), IVec2::new(12, 12), String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890$%., "));
    let font = Rc::from(font);

    let map_screen = Box::from(MapScreen::new(resolution, font.clone()));
    let game_screen = Box::from(GameScreen::new(resolution, font.clone()));
    let game_menu_screen = Box::from(GameMenuScreen::new(resolution, font.clone()));
    let services_screen = Box::from(ServicesScreen::new(resolution, font.clone()));
    let settings_screen = Box::from(SettingsScreen::new(resolution, font.clone()));
    let control_settings_screen = Box::from(ControlSettingsScreen::new(resolution, font.clone()));

    let gas_stations_screen = Box::from(ServiceSelectScreen::<GasStation>::new(resolution, font.clone()));
    let hostels_screen = Box::from(ServiceSelectScreen::<Hostel>::new(resolution, font.clone()));
    let repair_stations_screen = Box::from(ServiceSelectScreen::<RepairStation>::new(resolution, font.clone()));
    let shops_screen = Box::from(ServiceSelectScreen::<Shop>::new(resolution, font.clone()));

    ui_screens.insert(Screen::Map, map_screen);
    ui_screens.insert(Screen::Game, game_screen);
    ui_screens.insert(Screen::GameMenu, game_menu_screen);
    ui_screens.insert(Screen::Services, services_screen);
    ui_screens.insert(Screen::Settings, settings_screen);
    ui_screens.insert(Screen::ControlSettings, control_settings_screen);

    ui_screens.insert(Screen::GasStations, gas_stations_screen);
    ui_screens.insert(Screen::Hostels, hostels_screen);
    ui_screens.insert(Screen::RepairStations, repair_stations_screen);
    ui_screens.insert(Screen::Shops, shops_screen);

    ui_screens
}

pub trait UIScreen {
    fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent>;
    fn init(&mut self, game : &Game);
    fn render(&self, buffer : &mut RgbImage);
}