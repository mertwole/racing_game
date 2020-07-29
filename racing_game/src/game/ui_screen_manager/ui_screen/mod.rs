use image::{RgbImage};

use crate::game::Game;

mod map_screen;
pub use map_screen::*;

pub trait UIScreen {
    fn update(&mut self, game : &Game);
    fn render(&self, game : &Game, buffer : &mut RgbImage);
}