use crate::{boundary::{self, Boundary}, paddle::Paddle};

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self{ x, y }
    }
}

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self{ x, y }
    }
}

pub struct Projectile {
    pub position: Point,
    pub velocity: Velocity,
    pub boundary: Boundary,
}

impl Projectile {
    pub fn new(x: u32, y: u32, vx: i32, vy: i32, boundary: Boundary) -> Self {
        Self {
            position: Point::new(x, y),
            velocity: Velocity::new(vx, vy),
            boundary,
        }
    }

    pub fn predict_future_position(&mut self) -> (u32, u32) {
        self.check_collisions();
        let new_x = self.position.x.wrapping_add_signed(self.velocity.x);
        let new_y = self.position.y.wrapping_add_signed(self.velocity.y);

        (new_x, new_y)
    }

    pub fn fly_projectile(&mut self) {
        self.position.x = self.position.x.wrapping_add_signed(self.velocity.x);
        self.position.y = self.position.y.wrapping_add_signed(self.velocity.y);
    }

    fn check_collisions(&mut self) {
        let new_x = (self.position.x as i32) + self.velocity.x;
        let new_y = (self.position.y as i32) + self.velocity.y;

        let hits_top = new_y <= self.boundary.top() as i32;
        let hits_left = new_x <= self.boundary.left() as i32;
        let hits_right = new_x >= self.boundary.right() as i32;

        if hits_top {
            self.velocity.y *= -1;
        }
        if hits_left || hits_right {
            self.velocity.x *= -1;
        }
    }
}