mod command;
mod direction;
mod game;
mod marker;
mod paddle;

use crate::game::Game;
use std::io::stdout;

fn main() {
    Game::new(stdout(), 20, 15).run();
}