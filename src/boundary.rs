#[derive(Debug, Clone)]
pub struct Boundary {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

impl Boundary {
    pub fn new<T: Into<u32>>(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
            top: top.into(),
            bottom: bottom.into(),
        }
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn bottom(&self) -> u32 {
        self.bottom
    }
}