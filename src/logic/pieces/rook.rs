use crate::logic::basic::Coordinate;
use crate::logic::board::Board;

use super::queen::{all_moves_straight, is_straight, piece_between_straight};
use super::{is_friendly_fire, Move, MoveError};

pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    all_moves_straight(board, from)
}

pub fn move_piece(board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
    match a_move {
        Move::Promotion(..) => Err(MoveError::IllegalMove),
        Move::Regular(to) => {
            if from == to {
                return Err(MoveError::IllegalMove);
            }

            if is_friendly_fire(board, to) {
                return Err(MoveError::IllegalMove);
            }

            if is_straight(from, to) && !piece_between_straight(board, from, to) {
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                Ok(new_board)
            } else {
                Err(MoveError::IllegalMove)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::board::Board;
    use crate::logic::pieces::tests::{assert_all_moves_valid, assert_valid_in_all_moves};
    use crate::logic::pieces::Piece;

    use super::{all_moves, move_piece};

    fn test_board() -> Board {
        Board::from_configuration([
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', 'P', ' ', 'r', ' '],
            [' ', ' ', 'q', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            ['R', ' ', 'k', ' ', ' ', 'p', ' ', ' '],
            [' ', ' ', ' ', 'K', ' ', ' ', ' ', 'p'],
            [' ', 'P', 'P', 'P', ' ', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ])
    }

    #[test]
    fn test_all_moves_are_valid() {
        let board = test_board();
        assert_all_moves_valid(
            &board,
            Piece::from_letter('R').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('r').unwrap(),
            move_piece,
            all_moves,
        );
    }

    #[test]
    fn test_all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('R').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('r').unwrap(),
            move_piece,
            all_moves,
        );
    }
}
