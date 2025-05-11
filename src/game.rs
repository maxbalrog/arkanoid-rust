use std::io::Stdout;

use crossterm::terminal::size;

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        Self { 
            stdout,
            original_terminal_size,
            width,
            height, 
        }
    }

    pub fn run(&mut self) {
        println!("Game is running!");
    }
}