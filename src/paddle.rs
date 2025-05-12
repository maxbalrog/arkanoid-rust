use crate::direction::Direction;
use crate::marker::Integer;

pub struct Boundary {
    left: i32,
    right: i32
}

impl Boundary {
    pub fn new<T: Into<i32>>(left: T, right: T) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

pub struct Paddle {
    pub body: Vec<i32>,
    body_length: usize,
    boundary: Boundary,
}

impl Paddle {
    pub fn new(body_length: usize, boundary: Boundary) -> Self {
        let mid = (boundary.left + boundary.right) / 2;
        let half_body = (body_length / 2) as i32;
        let body = (-half_body..=half_body).map(|x| x + mid).collect();

        Self {
            body,
            body_length,
            boundary,
        }
    }

    fn move_within_boundaries(&mut self, times: i32) {
        let mut transform: bool = false;
        
        if times > 0 {
            if self.body[self.body_length-1] + times < self.boundary.right + 1 {
                transform = true;
            }
        } else if times < 0 {
            if self.body[0] + times > self.boundary.left {
                transform = true;
            }
        }

        if transform {
            self.body = self.body.iter().map(|x| x + times).collect()
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_within_boundaries(-1),
            Direction::Right => self.move_within_boundaries(1),
        }
    }
}