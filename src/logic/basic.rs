use std::fmt;


pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}


impl fmt::Display for Coordinate {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}
