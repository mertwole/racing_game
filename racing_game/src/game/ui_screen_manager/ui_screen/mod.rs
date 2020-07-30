use image::{RgbImage};

use crate::game::{Game, InputEvent, EventType};

mod map_screen;
pub use map_screen::*;

pub trait UIScreen {
    fn update(&mut self, game : &Game);
    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>);
    fn render(&self, game : &Game, buffer : &mut RgbImage);
}