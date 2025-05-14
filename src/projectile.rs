use crate::{boundary::{self, Boundary}, paddle::Paddle};

pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self{ x, y }
    }
}

struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self{ x, y }
    }
}

pub struct Projectile {
    position: Point,
    velocity: Velocity,
    boundary: Boundary,
    paddle: Paddle,
}

impl Projectile {
    pub fn new(x: u32, y: u32, vx: i32, vy: i32,
               boundary: Boundary, paddle: Paddle) -> Self {
        Self {
            position: Point::new(x, y),
            velocity: Velocity::new(vx, vy),
            boundary,
            paddle,
        }
    }

    pub fn fly(&mut self) {
        self.check_collisions();
        self.position.x = self.position.x.wrapping_add_signed(self.velocity.x);
        self.position.y = self.position.y.wrapping_add_signed(self.velocity.y);
    }

    fn check_collisions(&mut self) {
        let new_x = (self.position.x as i32) + self.velocity.x;
        let new_y = (self.position.y as i32) + self.velocity.y;

        let hits_top = new_x > self.boundary.top() as i32;
        let hits_bottom =  new_x < self.boundary.bottom() as i32;
        let hits_left = new_y < self.boundary.left() as i32;
        let hits_right = new_y > self.boundary.right() as i32;

        if hits_top || hits_bottom {
            self.velocity.x = -self.velocity.x;
        }
        if hits_left || hits_right {
            self.velocity.y = -self.velocity.y;
        }
    }
}