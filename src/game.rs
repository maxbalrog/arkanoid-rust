use std::{io::Stdout, time::Instant};
use std::thread;
use std::time::Duration;

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize}
};
use rand::{self, Rng};

use crate::{command::Command, direction::Direction, paddle::Paddle};
use crate::boundary::Boundary;
use crate::obstacle::{self, Obstacle};
use crate::projectile::{Point, Projectile};

const PADDLE_LENGTH: usize = 5;
const HAT_SIZE: u16 = 2;
const LAST_LVL: u8 = 2;

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    boundary: Boundary,
    paddle: Paddle,
    projectile: Projectile,
    obstacle: Obstacle,
    score: u32,
    lives: u8,
    lvl: u8,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        let boundary = Boundary::new(0, width, HAT_SIZE, HAT_SIZE + height + 1);
        let paddle = Paddle::new(PADDLE_LENGTH, boundary.clone());

        // spawn projectile
        let projectile = Game::spawn_projectile(width, height, boundary.clone());

        let obstacle = Obstacle::new(width, height, 1);

        Self { 
            stdout,
            original_terminal_size,
            width,
            height,
            boundary,
            paddle,
            projectile,
            obstacle,
            score: 0,
            lives: 3,
            lvl: 1,
        }
    }

    pub fn run(&mut self) {
        self.prepare_ui();
        self.render();
        
        let mut done: bool = false;
        let mut remaining_blocks = self.obstacle.body.len() > 0;
        while !done && remaining_blocks {
            let interval = Duration::from_millis(250);
            let now = Instant::now();

            done = self.wait_for_command(now, interval);
            self.render();

            let (projectile_lost, block_destroyed) = self.projectile.fly_projectile(&self.paddle, &mut self.obstacle);
            if block_destroyed {self.score += 10};

            let sleep_time = interval.abs_diff(now.elapsed());
            thread::sleep(sleep_time);
            self.render();

            if projectile_lost {
                self.lives -= 1;
                if self.lives > 0 {
                    self.projectile = Game::spawn_projectile(self.width, self.height, self.boundary.clone());
                } else {
                    done = true;
                }
            }
            remaining_blocks = self.obstacle.body.len() > 0;

            if !remaining_blocks {
                done = self.transition_to_next_lvl();
                remaining_blocks = self.obstacle.body.len() > 0;
            }
        }

        self.restore_ui();
        println!("Game over! Your score is {}", self.score);
    }

    fn wait_for_command(&mut self, now: Instant, interval: Duration) -> bool {
        let mut done = false;

        while now.elapsed() < interval {
            if let Some(command) = self.get_command(interval - now.elapsed()) {
                match command {
                    Command::Quit => {
                        done = true;
                        break;
                    }
                    Command::Move(direction) => {
                        self.paddle.shift(direction);
                    }
                }
            }
        }

        done
    }

    fn transition_to_next_lvl(&mut self) -> bool {
        let mut last_lvl = false;

        if self.lvl < LAST_LVL {
            self.obstacle = Obstacle::new(self.width, self.height, self.lvl+1);
        } else {
            last_lvl = true;
            self.draw_victory_screen();
        }

        last_lvl
    }

    fn draw_victory_screen(&mut self) {
        self.draw_background();
        self.draw_victory_text();

        let sleep_time = Duration::from_secs(30);
        thread::sleep(sleep_time);
    }

    fn spawn_projectile(width: u16, height: u16, boundary: Boundary) -> Projectile {
        let mut rng = rand::rng();
        let proj_x = rng.random_range(1..width-1) as u32;
        let proj_y = (height/2) as u32;

        let move_right = rng.random_bool(0.5);
        let vx = if move_right {1} else {-1};

        Projectile::new(proj_x, proj_y, vx, 1, boundary)
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') => 
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                }
            KeyCode::Right => Some(Command::Move(Direction::Right)),
            KeyCode::Left => Some(Command::Move(Direction::Left)),
            _ => None
        }
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
    }

    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3 + HAT_SIZE)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn restore_ui(&mut self) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Show).unwrap()
            .execute(ResetColor).unwrap();
        disable_raw_mode().unwrap();
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_borders();
        self.draw_paddle();
        self.draw_text_ui();
        self.draw_projectile();
        self.draw_obstacle();
    }

    fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 1 + HAT_SIZE..self.height + 1 + HAT_SIZE {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }

        // Also clear lives bar
        for x in self.width/2..self.width+1 {
            self.stdout
                .execute(MoveTo(x, 1)).unwrap()
                .execute(Print(" ")).unwrap();
        }
    }

    fn draw_borders(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();

        for y in HAT_SIZE..self.height + 2 + HAT_SIZE {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("#")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, HAT_SIZE)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(x, self.height + 1 + HAT_SIZE)).unwrap()
                .execute(Print("#")).unwrap();
        }
    }

    fn draw_paddle(&mut self) {
        let fg = SetForegroundColor(Color::Green);
        self.stdout.execute(fg).unwrap();

        for x in &self.paddle.body {
            self.stdout
                .execute(MoveTo(*x as u16, self.height + HAT_SIZE)).unwrap()
                .execute(Print("=")).unwrap();
        }
    }

    fn draw_text_ui(&mut self) {
        let fg = SetForegroundColor(Color::White);
        self.stdout.execute(fg).unwrap();

        // draw scores
        self.stdout
            .execute(MoveTo(0, 1)).unwrap()
            .execute(Print(format!("SCORE: {:04}", self.score))).unwrap();

        // draw lives
        let lives_str = "❤ ".repeat(self.lives as usize);
        self.stdout
            .execute(MoveTo(self.width - 1 - 3, 1)).unwrap()
            .execute(Print(format!("{:>6}", lives_str))).unwrap();
    }

    fn draw_projectile(&mut self) {
        let fg = SetForegroundColor(Color::White);
        self.stdout.execute(fg).unwrap();

        let x = self.projectile.position.x as u16;
        let y = self.projectile.position.y as u16;
        self.stdout
            .execute(MoveTo(x, y)).unwrap()
            .execute(Print("●")).unwrap();
    }

    fn draw_obstacle(&mut self) {
        let fg = SetForegroundColor(Color::Green);
        self.stdout.execute(fg).unwrap();

        for Point{ x, y } in &self.obstacle.body {
            self.stdout
                .execute(MoveTo(*x as u16, *y as u16)).unwrap()
                .execute(Print("■")).unwrap();
        }
    }

    fn draw_victory_text(&mut self) {
        let fg = SetForegroundColor(Color::White);
        self.stdout.execute(fg).unwrap();

        self.stdout
            .execute(MoveTo(self.width/2, self.height/2)).unwrap()
            .execute(Print("YOU WON!")).unwrap();
    }

}