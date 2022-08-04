pub mod pawn;

use std::error::Error;
use std::fmt;

use super::basic::Player;
use super::basic::Coordinate;
use super::board::Board;
use crate::utils::ValueError;


#[derive(Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}


impl PieceType {
    pub fn from_letter(letter: char) -> Result<Self, ValueError> {
        Ok(match letter {
            'K' => Self::King,
            'Q' => Self::Queen,
            'R' => Self::Rook,
            'B' => Self::Bishop,
            'N' => Self::Knight,
            'P' => Self::Pawn,
            _ => Err(ValueError)?
        })
    }

    pub fn get_symbol(&self, player: &Player) -> &str {
        match player {
            Player::White => match self {
                Self::King => "\u{2654}",
                Self::Queen => "\u{2655}",
                Self::Rook => "\u{2656}",
                Self::Bishop => "\u{2657}",
                Self::Knight => "\u{2658}",
                Self::Pawn => "\u{2659}",
            },
            Player::Black => match self {
                Self::King => "\u{265A}",
                Self::Queen => "\u{265B}",
                Self::Rook => "\u{265C}",
                Self::Bishop => "\u{265D}",
                Self::Knight => "\u{265E}",
                Self::Pawn => "\u{265F}",
            },
        }
    }

    pub fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        // Piece at `from` and `piece` is from player with turn already checked
        self.move_piece(board, from, to).is_ok()
    }

    pub fn move_piece(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<Board, MoveError> {
        // Piece at `from` and `piece` is from player with turn already checked
        match self {
            Self::Pawn => pawn::move_piece(board, from, to),
            _ => Err(MoveError),
        }
    }
}


#[derive(Clone)]
pub struct Piece {
    pub player: Player,
    pub piece_type: PieceType,
}


impl Piece {
    pub fn from_letter(letter: char) -> Result<Self, ValueError> {
        let player = match letter {
            'A'..='Z' => Player::White,
            'a'..='z' => Player::Black,
            _ => Err(ValueError)?,
        };
        
        let upper_letter = letter.to_ascii_uppercase();
        let piece_type = PieceType::from_letter(upper_letter)?;

        Ok(Self {
            player,
            piece_type,
        })
    }

    pub fn get_symbol(&self) -> &str {
        self.piece_type.get_symbol(&self.player)
    }

    pub fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        // Piece at `from` and already checked

        if self.player != board.turn {
            return false;
        }

        self.piece_type.can_move(board, from, to)
    }

    pub fn move_piece(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<Board, MoveError> {
        // Piece at `from` and already checked

        if self.player != board.turn {
            return Err(MoveError);
        }

        self.piece_type.move_piece(board, from, to)
    }
}


#[derive(Debug)]
pub struct MoveError;
impl Error for MoveError {}
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoveError")
    }
}
