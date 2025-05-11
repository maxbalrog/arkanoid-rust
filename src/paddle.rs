use crate::direction::Direction;

struct Boundary {
    left: i16,
    right: i16
}

struct Paddle {
    body: Vec<i16>,
    body_length: usize,
    boundary: Boundary,
}

impl Paddle {
    pub fn new(body_length: usize, boundary: Boundary) -> Self {
        let mid = (boundary.left + boundary.right) / 2;
        let half_body = (body_length / 2) as i16;
        let body = (-half_body..=half_body).map(|x| x + mid).collect();

        Self {
            body,
            body_length,
            boundary,
        }
    }

    fn move_within_boundaries(&mut self, times: i16) {
        let mut transform: bool = false;
        
        if times > 0 {
            if self.body[self.body_length] + times < self.boundary.right {
                transform = true;
            }
        } else if times < 0 {
            if self.body[self.body_length] + times > self.boundary.left {
                transform = true;
            }
        }

        if transform {
            self.body = self.body.iter().map(|x| x + times).collect()
        }
    }

    fn shift(&mut self, direction: Option<Direction>) {
        if let Some(direction) = direction {
            match direction {
                Direction::Left => self.move_within_boundaries(-1),
                Direction::Right => self.move_within_boundaries(1),
            }
        }
    }
}