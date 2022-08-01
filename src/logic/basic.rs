use crate::draw::text::{LABEL_WHITE, LABEL_BLACK};
use crate::logic::board::{BOARD_SIZE, BOARD_MAX_AXIS};
use crate::utils::ValueError;

use std::fmt;


pub struct Coordinate {
    x: usize,
    y: usize,
}


impl Coordinate {
    pub fn try_new(x: usize, y: usize) -> Result<Self, ValueError> {
        Ok(Self {
            x: Self::try_axis_bound(x)?,
            y: Self::try_axis_bound(y)?,
        })
    }

    pub fn try_axis_bound(val: usize) -> Result<usize, ValueError> {
        match val {
            x @ 0..=BOARD_MAX_AXIS => Ok(val),
            _ => Err(ValueError),
        }
    }

    pub fn xv(&self) -> usize { self.x }
    pub fn yv(&self) -> usize { self.y }

    pub fn to_field_name(&self) -> String {
        let column = (self.x + ('A' as usize)) as u8 as char;
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
