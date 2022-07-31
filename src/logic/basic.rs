use crate::draw::text::{LABEL_WHITE, LABEL_BLACK};
use crate::logic::board::BOARD_SIZE;

use std::fmt;


pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}


impl Coordinate {
    pub fn to_field_name(&self) -> String {
        // TODO: Make this safe
        let column = (self.x + ('A' as u16)) as u8 as char;
        format!("{}{}", column, BOARD_SIZE - self.y)
    }
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


impl Player {
    pub fn to_label(&self) -> &str {
        match self {
            Player::White => LABEL_WHITE,
            Player::Black => LABEL_BLACK,
        }
    }
}
