extern crate image;

mod render;
mod window;
mod input;
mod game;

use game::*;

fn main() {
    let mut game = Game::new();
    game.enter_gameloop();
}
