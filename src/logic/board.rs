use crate::logic::piece::{self, Piece, PieceType};
use crate::logic::basic::Player;
use crate::utils::DiscreetUnwrap;

use std::fmt;


pub const BOARD_SIZE: u16 = 8;
pub const BOARD_SIZE_USIZE: usize = 8;


pub enum TileContent {
    Empty,
    Piece(Piece),
}


impl TileContent {
    pub fn from_letter(letter: char) -> Self {
        if letter == ' ' {
            return Self::Empty
        }

        let player = match letter {
            'A'..='Z' => Player::White,
            'a'..='z' => Player::Black,
            _ => panic!("Invalid letter group"),
        };
        
        let upper_letter = letter.to_ascii_uppercase();

        let piece_type: Box<dyn PieceType> = match upper_letter {
            'K' => Box::new(piece::King::new()),
            'Q' => Box::new(piece::Queen::new()),
            'R' => Box::new(piece::Rook::new()),
            'B' => Box::new(piece::Bishop::new()),
            'N' => Box::new(piece::Knight::new()),
            'P' => Box::new(piece::Pawn::new()),
            _ => panic!("Invalid letter"),
        };

        Self::Piece(Piece { piece_type,  player, })
    }
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
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


pub struct Board {
    pub tiles: [[TileContent; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
    pub turn: Player,
}


impl Board {
    pub fn default() -> Self {
        let tiles = DEFAULT_PIECE_CONFIGURATION
            .into_iter()
            .map(|row| {
                row
                    .into_iter()
                    .map(|letter| {
                        TileContent::from_letter(letter)
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .duwrp()
            })
            .collect::<Vec<_>>()
            .try_into()
            .duwrp();

        Self {
            tiles,
            turn: Player::White,
        }
    }

    pub fn get_tile(& self, coordinate: &Coordinate) -> &TileContent {
        self.assert_coordinate(coordinate);
        &self.tiles[coordinate.y as usize][coordinate.x as usize]
    }

    pub fn coordinate_to_fieldname(& self, coordinate: &Coordinate) -> FieldName {
        self.assert_coordinate(coordinate);

        let hpos = coordinate.x;
        let vpos = BOARD_SIZE - coordinate.y - 1;

        FieldName {
            horizontal: offset_char('A', hpos.try_into().unwrap()).to_string(),
            vertical:  offset_char('1', vpos.try_into().unwrap()).to_string(),
        }
    }

    pub fn get_field_color_at(& self, coordinate: &Coordinate) -> FieldColor {
        self.assert_coordinate(coordinate);

        match (coordinate.x + coordinate.y) % 2 {
            0 => FieldColor::White,
            1 => FieldColor::Black,
            _ => panic!("Unreachable"),
        }
    }

    fn assert_coordinate(& self, coordinate: &Coordinate) {
        if coordinate.x > BOARD_SIZE || coordinate.y > BOARD_SIZE {
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


const DEFAULT_PIECE_CONFIGURATION: [[char; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE] = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
];
