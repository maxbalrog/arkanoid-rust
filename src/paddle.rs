use crate::boundary::Boundary;
use crate::direction::Direction;

pub struct Paddle {
    pub body: Vec<u32>,
    body_length: usize,
    boundary: Boundary,
}

impl Paddle {
    pub fn new(body_length: usize, boundary: Boundary) -> Self {
        let mid = (boundary.left() + boundary.right()) / 2;
        let half_body = (body_length / 2) as i32;
        let body = (-half_body..=half_body).map(|x| mid.wrapping_add_signed(x)).collect();

        Self {
            body,
            body_length,
            boundary,
        }
    }

    fn move_within_boundaries(&mut self, times: i32) {
        let mut transform: bool = false;
        
        if times > 0 {
            let new_right = self.body[self.body_length-1].wrapping_add_signed(times);
            if new_right < self.boundary.right() + 1 {
                transform = true;
            }
        } else if times < 0 {
            let new_left = self.body[0].wrapping_add_signed(times);
            if new_left > self.boundary.left() {
                transform = true;
            }
        }

        if transform {
            self.body = self.body.iter().map(|x| x.wrapping_add_signed(times)).collect()
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_within_boundaries(-1),
            Direction::Right => self.move_within_boundaries(1),
        }
    }
}