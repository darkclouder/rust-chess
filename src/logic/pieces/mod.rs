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
            _ => Err(MoveError::IllegalMove),
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
            return Err(MoveError::IllegalMove);
        }

        self.piece_type.move_piece(board, from, to)
    }
}


#[derive(Debug)]
pub enum MoveError {
    IllegalMove,
    PromotionRequired,
}
impl Error for MoveError {}
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoveError")
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::logic::basic::Coordinate;
    use crate::logic::board::{Board, BOARD_SIZE};

    use super::MoveError;

    type MovePieceFn = fn(&Board, &Coordinate, &Coordinate) -> Result<Board, MoveError>;
    type AllMovesFn = fn(&Board, &Coordinate) -> Vec<Coordinate>;

    pub fn assert_vecs_same_elements<T, F, K>(actual: &mut Vec<T>, expected: &mut Vec<T>, keyf: &F)
    where
        T: Clone + Debug + PartialEq,
        F: Fn(&T) -> K,
        K: Ord,
    {
        assert!(actual.len() == expected.len());
        let mut actual_sorted = actual.clone();
        actual_sorted.sort_by_key(keyf);
        let mut expected_sorted = expected.clone();
        expected_sorted.sort_by_key(keyf);

        for (actual_item, expected_item) in actual_sorted.iter().zip(expected_sorted.iter()) {
            assert_eq!(*actual_item, *expected_item);
        }
    }

    pub fn assert_all_moves_valid(
        board: &Board,
        move_piece: MovePieceFn,
        all_moves: AllMovesFn,
    ) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let from = c(x, y);
                assert_all_moves_valid_from(
                    board,
                    &from,
                    &all_moves(board, &from),
                    move_piece,
                );
            }
        }
    }

    fn assert_all_moves_valid_from(
        board: &Board,
        from: &Coordinate,
        moves: &Vec<Coordinate>,
        move_piece: MovePieceFn
    ) {
        for to in moves {
            if let Err(e) = move_piece(board, from, to) {
                panic!(
                    "Could not move from {} to {} as {:?}: {}",
                    from, to, board.turn, e
                );
            }
        }
    }


    pub fn assert_valid_in_all_moves(
        board: &Board,
        move_piece: MovePieceFn,
        all_moves: AllMovesFn
    ) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                assert_valid_in_all_moves_from(
                    board,
                    &c(x,y),
                    move_piece,
                    all_moves,
                );
            }
        }
    }


    fn assert_valid_in_all_moves_from(
        board: &Board,
        from: &Coordinate,
        move_piece: MovePieceFn,
        all_moves: AllMovesFn
    ) {
        let mut valid_moves: Vec<Coordinate> = Vec::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let to = c(x, y);

                if move_piece(board, from, &to).is_ok() {
                    valid_moves.push(to);
                }
            }
        }

        assert_vecs_same_elements(
            &mut valid_moves, 
            &mut all_moves(board, from),
            &|c: &Coordinate| (c.xv(), c.yv()),
        )
    }

    pub fn c(x: usize, y: usize) -> Coordinate {
        Coordinate::try_new(x, y).unwrap()
    }
}
