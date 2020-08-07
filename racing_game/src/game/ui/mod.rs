use std::collections::HashMap;
use std::rc::Rc;

use image::RgbImage;

use crate::engine::common::IVec2;
use super::{Game, EventType, InputEvent, Player};
use super::services::*;

mod ui_screen;
use ui_screen::*;

pub enum UIEvent{
    StartRide,
    SelectCityDestination(usize),
    ChangePlayer(Player),
    ChangeScreen(Screen),
    ServiceAction(ServiceId, ServiceAction)
}

pub struct UI {
    game : Option<Rc<Game>>,
    ui_screens : HashMap<Screen, Box<dyn UIScreen>>,
    current_screen : Screen,
}

impl UI {
    pub fn new(resolution : &IVec2) -> UI {
        let ui_screens = create_all_screens(resolution);
        UI { ui_screens, current_screen : Screen::Services, game : None } 
    }

    pub fn set_game(&mut self, game : Rc<Game>) {
        self.game = Some(game);
        self.ui_screens.get_mut(&self.current_screen).unwrap().init(self.game.as_ref().unwrap());
    }

    fn change_screen(&mut self, screen : Screen) {
        self.current_screen = screen;
        self.ui_screens.get_mut(&screen).unwrap().init(self.game.as_ref().unwrap());
    }

    pub fn enter_city(&mut self) {
        self.change_screen(Screen::Map);
    }

    pub fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        let events = self.ui_screens.get_mut(&self.current_screen).unwrap().update(input, delta_time);

        events.into_iter()
        .filter(|event| {
            match event {
                UIEvent::ChangeScreen(screen) => { self.change_screen(*screen); false }
                _ => { true }
            }
        })
        .collect()
    }

    pub fn render(&mut self, buffer : &mut RgbImage) {
        self.ui_screens.get(&self.current_screen).unwrap().render(buffer);
    }
}