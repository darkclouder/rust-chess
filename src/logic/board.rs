use crate::logic::piece::Piece;
use crate::logic::basic::Player;

use std::fmt;


pub enum TileContent {
    Empty,
    Occupied(Piece),
}


pub enum FieldColor {
    White,
    Black,
}


pub struct FieldName {
    pub horizontal: String,
    pub vertical: String,
}


pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}


impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


pub struct Board {
    pub tiles: [[TileContent; 8]; 8],
    pub turn: Player,
    pub size: u16,
}


impl Board {
    pub fn default() -> Self {
        let tiles: [[TileContent; 8]; 8] = array_init::array_init(
            |_| array_init::array_init(|_| TileContent::Empty)
        );

        Self {
            tiles,
            turn: Player::White,
            size: 8,
        }
    }

    pub fn coordinate_to_fieldname(& self, coordinate: &Coordinate) -> FieldName {
        self.assert_coordinate(coordinate);

        FieldName {
            horizontal: offset_char('A', coordinate.x.try_into().unwrap()).to_string(),
            vertical:  offset_char('1', coordinate.y.try_into().unwrap()).to_string(),
        }
    }

    pub fn get_field_color_at(& self, coordinate: &Coordinate) -> FieldColor {
        self.assert_coordinate(coordinate);

        match (coordinate.x + coordinate.y) % 2 {
            0 => FieldColor::Black,
            1 => FieldColor::White,
            _ => panic!("Unreachable"),
        }
    }

    fn assert_coordinate(& self, coordinate: &Coordinate) {
        if coordinate.x > self.size || coordinate.y > self.size {
            panic!("Coordinate out of bound: {}", coordinate);
        }
    }
}


fn offset_char(c: char, n: i8) -> char {
    assert!(c.is_ascii_alphanumeric());
    let ret = ((c as i8) + n) as u8 as char;
    assert!(ret.is_ascii_alphanumeric());
    ret
}
