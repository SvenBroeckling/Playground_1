extern crate sdl3;
mod game;
mod scenes;
mod stars;
mod viewport;

use game::Game;

fn main() {
    let mut game = Game::new("Test", 800, 600);
    game.run();
}
