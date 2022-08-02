use crate::logic::basic::{Coordinate, Player};
use crate::logic::piece::{self, Piece, PieceType};
use crate::utils::DiscreetUnwrap;


pub const BOARD_SIZE: usize = 8;
pub const BOARD_MAX_AXIS: usize = BOARD_SIZE - 1;


pub struct Board {
    pub tiles: [[TileContent; BOARD_SIZE]; BOARD_SIZE],
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
        &self.tiles[coordinate.yv()][coordinate.xv()]
    }
}


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


const DEFAULT_PIECE_CONFIGURATION: [[char; BOARD_SIZE]; BOARD_SIZE] = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
];
