use crate::{boundary::Boundary, obstacle::Obstacle, paddle::Paddle};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    pub fn fly_projectile(&mut self, paddle: &Paddle, obstacle: &mut Obstacle) -> (bool, bool) {
        let projectile_lost = self.check_paddle_collision(paddle);
        let block_destroyed = self.check_obstacle_collision(obstacle);
        self.assign_new_position();

        (projectile_lost, block_destroyed)
    }

    fn predict_future_position(&mut self) -> (u32, u32) {
        self.check_wall_collision();
        let new_x = self.position.x.wrapping_add_signed(self.velocity.x);
        let new_y = self.position.y.wrapping_add_signed(self.velocity.y);

        (new_x, new_y)
    }

    fn assign_new_position(&mut self) {
        self.position.x = self.position.x.wrapping_add_signed(self.velocity.x);
        self.position.y = self.position.y.wrapping_add_signed(self.velocity.y);
    }

    fn check_paddle_collision(&mut self, paddle: &Paddle) -> bool {
        let (proj_x, proj_y) = self.predict_future_position();
        let mut projectile_lost = false;

        if proj_y == self.boundary.bottom() - 1 {
            let mut collided_with_paddle = false;
            for paddle_x in &paddle.body {
                if proj_x == *paddle_x { 
                    collided_with_paddle = true;
                    break;
                }
            }

            if collided_with_paddle {
                self.velocity.y *= -1;
            } else {
                projectile_lost = true;
            }
        }

        projectile_lost
    }

    fn check_obstacle_collision(&mut self, obstacle: &mut Obstacle) -> bool {
        let (proj_x, proj_y) = self.predict_future_position();
        let mut block_destroyed = false;

        let proj = Point::new(proj_x, proj_y);
        let idx = obstacle.body.iter().position(|n| n == &proj);

        if let Some(idx) = idx {
            obstacle.body.remove(idx);
            block_destroyed = true;
            self.velocity.y *= -1;
        }

        block_destroyed
    }

    fn check_wall_collision(&mut self) {
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