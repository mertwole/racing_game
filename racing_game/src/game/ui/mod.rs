use std::collections::HashMap;
use std::rc::Rc;

use image::RgbImage;

use crate::engine::common::IVec2;
use super::{Game, EventType, InputEvent, Player};
use crate::engine::window::Key;
use super::services::*;

mod ui_screen;
use ui_screen::*;

pub enum UIEvent{
    StartRide,
    SelectCityDestination(usize),
    ChangePlayer(Player),
    ChangeScreen(Screen),
    PreviousScreen,
    ServiceAction(ServiceId, ServiceAction),
    SetRidePaused(bool),
    BindKey(InputEvent, Key)
}

pub struct UI {
    game : Option<Rc<Game>>,
    ui_screens : HashMap<Screen, Box<dyn UIScreen>>,
    screen_stack : Vec<Screen>
}

impl UI {
    pub fn new(resolution : &IVec2) -> UI {
        let ui_screens = create_all_screens(resolution);
        UI { ui_screens, game : None, screen_stack : vec![Screen::Services] } 
    }

    pub fn set_game(&mut self, game : Rc<Game>) {
        self.game = Some(game);
        self.ui_screens.get_mut(&self.screen_stack[self.screen_stack.len() - 1]).unwrap().init(self.game.as_ref().unwrap());
    }

    fn change_screen(&mut self, screen : Screen) {
        self.screen_stack.push(screen);
        self.ui_screens.get_mut(&screen).unwrap().init(self.game.as_ref().unwrap());
    }

    fn prev_screen(&mut self) {
        if self.screen_stack.len() > 1 {
            self.screen_stack.pop();    
        } else { 
            panic!("there is no previous screen to go!"); 
        }
    }

    pub fn enter_city(&mut self) {
        self.change_screen(Screen::Services);
    }

    pub fn update(&mut self, input : &Vec<(InputEvent, EventType)>, delta_time : f32) -> Vec<UIEvent> {
        let events = self.ui_screens.get_mut(&self.screen_stack[self.screen_stack.len() - 1]).unwrap().update(input, delta_time);

        events.into_iter()
        .filter(|event| {
            match event {
                UIEvent::ChangeScreen(screen) => { self.change_screen(*screen); false }
                UIEvent::PreviousScreen => { self.prev_screen(); false }
                _ => { true }
            }
        })
        .collect()
    }

    pub fn render(&mut self, buffer : &mut RgbImage) {
        self.ui_screens.get(&self.screen_stack[self.screen_stack.len() - 1]).unwrap().render(buffer);
    }
}