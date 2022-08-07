pub mod pawn;
pub mod queen;
pub mod rook;
pub mod bishop;

use std::error::Error;
use std::fmt;

use super::basic::Player;
use super::basic::Coordinate;
use super::board::Board;
use crate::utils::ValueError;


#[derive(Debug, PartialEq, Eq, Clone)]
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
        match self.move_piece(board, from, &Move::Regular(to.clone())) {
            Ok(_) => true,
            Err(MoveError::PromotionRequired) => true,
            _ => false,
        }
    }

    pub fn move_piece(&self, board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
        // Piece at `from` and `piece` is from player with turn already checked
        match self {
            Self::Pawn => pawn::move_piece(board, from, a_move),
            Self::Queen => queen::move_piece(board, from, a_move),
            Self::Rook => rook::move_piece(board, from, a_move),
            Self::Bishop => bishop::move_piece(board, from, a_move),
            _ => Err(MoveError::IllegalMove),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn move_piece(&self, board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
        // Piece at `from` and already checked

        if self.player != board.turn {
            return Err(MoveError::IllegalMove);
        }

        self.piece_type.move_piece(board, from, a_move)
    }

    pub fn promoted(&self, new_type: PieceType) -> Self {
        let mut new_piece = self.clone();
        new_piece.piece_type = new_type;
        new_piece
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Move {
    Regular(Coordinate),
    Promotion(Coordinate, PieceType),
}


impl Move {
    pub fn get_to(&self) -> &Coordinate {
        match self {
            Move::Regular(to) => &to,
            Move::Promotion(to, _) => &to,
        }
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
    use crate::logic::board::{Board, BOARD_SIZE, TileContent};

    use super::{MoveError, Move, PieceType, Piece};

    type MovePieceFn = fn(&Board, &Coordinate, &Move) -> Result<Board, MoveError>;
    type AllMovesFn = fn(&Board, &Coordinate) -> Vec<Move>;

    pub fn assert_vecs_same_elements<T, F, K>(actual: &mut Vec<T>, expected: &mut Vec<T>, keyf: &F)
    where
        T: Clone + Debug + PartialEq,
        F: Fn(&T) -> K,
        K: Ord,
    {
        println!("{:?}", actual);
        println!("{:?}", expected);
        assert_eq!(actual.len(), expected.len());
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
        piece: Piece,
        move_piece: MovePieceFn,
        all_moves: AllMovesFn,
    ) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let from = c(x, y);

                let mut board_with_piece = board.clone();
                board_with_piece.set_tile(
                    &from,
                    TileContent::Piece(piece.clone()),
                );

                assert_all_moves_valid_from(
                    &board_with_piece,
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
        moves: &Vec<Move>,
        move_piece: MovePieceFn
    ) {
        for a_move in moves {
            match move_piece(board, from, a_move) {
                Ok(new_board) => {
                    assert!(matches!(new_board.get_tile(a_move.get_to()), TileContent::Piece(_)))
                },
                Err(e) => {
                    panic!(
                        "Could not move from {} to {:?} as {:?}: {}",
                        from, a_move, board.turn, e
                    );
                }
            };
        }
    }


    pub fn assert_valid_in_all_moves(
        board: &Board,
        piece: Piece,
        move_piece: MovePieceFn,
        all_moves: AllMovesFn,
    ) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let from = c(x, y);

                let mut board_with_piece = board.clone();
                board_with_piece.set_tile(&from, TileContent::Piece(piece.clone()));

                assert_valid_in_all_moves_from(
                    &board_with_piece,
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
        let mut valid_moves: Vec<Move> = Vec::new();

        println!("From {}", from);

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let to = c(x, y);

                let move_regular = Move::Regular(to.clone());
                if move_piece(board, from, &move_regular).is_ok() {
                    valid_moves.push(move_regular);
                }

                for piece_type in [
                    PieceType::King,
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                    PieceType::Pawn,
                ] {
                    let move_promotion = Move::Promotion(to.clone(), piece_type);
                    if move_piece(board, from, &move_promotion).is_ok() {
                        valid_moves.push(move_promotion);
                    }
                }
            }
        }

        assert_vecs_same_elements(
            &mut valid_moves, 
            &mut all_moves(board, from),
            &|m: &Move| match m {
                Move::Regular(to) => (to.xv(), to.yv(), "".to_string()),
                Move::Promotion(to, piece_type) => (to.xv(), to.yv(), format!("{:?}", piece_type)),
            },
        )
    }

    pub fn c(x: usize, y: usize) -> Coordinate {
        Coordinate::try_new(x, y).unwrap()
    }

    pub fn m(x: usize, y: usize) -> Move {
        Move::Regular(c(x, y))
    }
}
