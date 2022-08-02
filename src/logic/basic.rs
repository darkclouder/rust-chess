use crate::draw::text::{LABEL_WHITE, LABEL_BLACK};
use crate::logic::board::{BOARD_SIZE, BOARD_MAX_AXIS};
use crate::utils::ValueError;

use std::fmt;


pub enum FieldColor {
    White,
    Black,
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
            x @ 0..=BOARD_MAX_AXIS => Ok(x),
            _ => Err(ValueError),
        }
    }

    pub fn xv(&self) -> usize { self.x }
    pub fn yv(&self) -> usize { self.y }

    pub fn to_field_name(&self) -> String {
        format!("{}{}", column_to_name(self.x), row_to_name(self.y))
    }

    pub fn get_field_color(&self) -> FieldColor {
        match (self.x + self.y) % 2 {
            0 => FieldColor::White,
            1 => FieldColor::Black,
            _ => panic!("Unreachable"),
        }
    }
}


impl fmt::Display for Coordinate {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


pub fn row_to_name(y: usize) -> String {
    format!("{}", BOARD_SIZE.saturating_sub(y))
}


pub fn column_to_name(x: usize) -> String {
    let name = (x + ('A' as usize)) as u8 as char;
    assert!(name.is_ascii_alphanumeric());
    format!("{}", name)
}
