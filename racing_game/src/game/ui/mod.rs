use std::collections::HashMap;
use std::rc::Rc;

use image::RgbImage;

use crate::engine::common::IVec2;
use crate::engine::ui::font::*;
use super::{Game, EventType, InputEvent};

mod ui_screen;
use ui_screen::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Screen{
    Map,
    Game
}

pub enum UIEvent{
    StartRide,
    SelectCityDestination(usize),
    ChangeScreen(Screen)
}

pub struct UI {
    game : Option<Rc<Game>>,
    ui_screens : HashMap<Screen, Box<dyn UIScreen>>,
    current_screen : Screen,
}

impl UI {
    pub fn new(resoulution : &IVec2) -> UI {
        let mut ui_screens = HashMap::<Screen, Box<dyn UIScreen>>::new();

        let font = Font::new(Game::load_image_rgba("font.png"), IVec2::new(12, 12), String::from("ABCDEFGHIJ"));
        let font = Rc::from(font);

        let map_screen = Box::from(MapScreen::new(resoulution, font.clone()));
        let game_screen = Box::from(GameScreen::new(resoulution, font.clone()));

        ui_screens.insert(Screen::Map, map_screen);
        ui_screens.insert(Screen::Game, game_screen);

        UI { ui_screens, current_screen : Screen::Map, game : None } 
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

    pub fn update(&mut self, delta_time : f32) -> Vec<UIEvent> {
        let events = self.ui_screens.get_mut(&self.current_screen).unwrap().update(delta_time);

        events.into_iter()
        .filter(|event| {
            match event {
                UIEvent::ChangeScreen(screen) => { self.change_screen(*screen); false }
                _ => { true }
            }
        })
        .collect()
    }

    pub fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>) {
        self.ui_screens.get_mut(&self.current_screen).unwrap().process_input(input);
    }

    pub fn render(&mut self, buffer : &mut RgbImage) {
        self.ui_screens.get(&self.current_screen).unwrap().render(buffer);
    }
}