use crate::logic::basic::{Coordinate, Player};
use crate::logic::board::Board;

use std::{error::Error, fmt};


pub struct Piece {
    pub piece_type: Box<dyn PieceType>,
    pub player: Player,
}


impl Piece {
    pub fn get_symbol(&self) -> &str {
        self.piece_type.get_symbol(self)
    }

    pub fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        self.piece_type.can_move(board, from, to)
    }

    pub fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
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


pub trait PieceType {
    fn get_symbol(&self, piece: &Piece) -> &str;
    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool;
    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError>;
}


pub struct King {}
impl King {
    pub fn new() -> Self { Self {} }
}
impl PieceType for King {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2654}",
            Player::Black => "\u{265A}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => Ok(()),
        }
    }
}


pub struct Queen {}
impl Queen {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Queen {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2655}",
            Player::Black => "\u{265B}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => Ok(()),
        }
    }
}


pub struct Rook {}
impl Rook {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Rook {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2656}",
            Player::Black => "\u{265C}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => Ok(()),
        }
    }
}


pub struct Bishop {}
impl Bishop {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Bishop {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2657}",
            Player::Black => "\u{265D}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => Ok(()),
        }
    }
}


pub struct Knight {}
impl Knight {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Knight {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2658}",
            Player::Black => "\u{265E}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => Ok(()),
        }
    }
}


pub struct Pawn {
    has_moved: bool,
}
impl Pawn {
    pub fn new() -> Self {
        Self {
        has_moved: false
        }
    }
}
impl PieceType for Pawn {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2659}",
            Player::Black => "\u{265F}",
        }
    }

    fn can_move(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        false
    }

    fn move_piece(&mut self, board: &Board, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        match self.can_move(board, from, to) {
            false => Err(MoveError),
            true => {
                self.has_moved = true;
                Ok(())
            },
        }
    }
}
