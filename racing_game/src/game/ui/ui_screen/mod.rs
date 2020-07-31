use image::{RgbImage};

use crate::game::{Game, InputEvent, EventType};
use super::UIEvent;

mod map_screen;
pub use map_screen::*;

mod game_screen;
pub use game_screen::*;

pub trait UIScreen {
    fn update(&mut self, delta_time : f32) -> Vec<UIEvent>;
    fn init(&mut self, game : &Game);
    fn process_input(&mut self, input : &Vec<(InputEvent, EventType)>);
    fn render(&self, buffer : &mut RgbImage);
}