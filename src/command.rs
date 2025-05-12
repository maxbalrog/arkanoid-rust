use crate::direction::Direction;

pub enum Command {
    Quit,
    Move(Direction)
}