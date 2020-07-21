extern crate image;

mod engine;
mod game;

use game::*;

fn main() {
    let mut game = Game::new();
    game.enter_gameloop();
}
