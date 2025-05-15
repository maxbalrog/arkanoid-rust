use crate::projectile::Point;

pub struct Obstacle {
    pub body: Vec<Point>,
}

impl Obstacle {
    pub fn new(width: u16, height: u16, lvl: u8) -> Self {
        Obstacle {
            body: Obstacle::generate_lvl(width, height, lvl)
        }
    }

    fn generate_lvl(width: u16, height: u16, lvl: u8) -> Vec<Point> {
        match lvl {
            1 => Obstacle::generate_lvl_1(width, height),
            2 => Obstacle::generate_lvl_2(width, height),
            _ => panic!("level {lvl} is not available!"),
        }
    }

    fn generate_lvl_1(width: u16, height: u16) -> Vec<Point> {
        let mut body = vec![];
        for i in (1..width).step_by(2) {
            body.push(Point::new(i.into(), 4));
        };

        body
    }

    fn generate_lvl_2(width: u16, height: u16) -> Vec<Point> {
        vec![]
    }
}
