use crate::logic::basic::Coordinate;
use crate::logic::board::Board;

use super::queen::{all_moves_diagonal, is_diagonal, piece_between_diagonal};
use super::{Move, MoveError, is_friendly_fire};


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    all_moves_diagonal(board, from)
}


pub fn move_piece(board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
    return match a_move {
        Move::Promotion(..) => Err(MoveError::IllegalMove),
        Move::Regular(to) => {
            if from == to {
                return Err(MoveError::IllegalMove);
            }

            if is_friendly_fire(&board, &to) {
                return Err(MoveError::IllegalMove);
            }

            if is_diagonal(from, to) && !piece_between_diagonal(board, from, to) {
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                Ok(new_board)
            } else {
                return Err(MoveError::IllegalMove)
            }
        },
    };
}


#[cfg(test)]
mod tests {
    use crate::logic::board::Board;
    use crate::logic::pieces::Piece;
    use crate::logic::pieces::tests::{assert_all_moves_valid, assert_valid_in_all_moves, m, c};

    use super::{move_piece, all_moves};


    fn test_board() -> Board {
        Board::from_configuration([
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', 'P', ' ', 'B', ' '],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            [' ', ' ', 'k', ' ', ' ', 'p', ' ', ' '],
            [' ', ' ', ' ', 'K', ' ', ' ', ' ', 'p'],
            [' ', 'P', 'P', 'P', ' ', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ])
    }


    #[test]
    fn all_moves_are_valid() {
        let board = test_board();
        assert_all_moves_valid(
            &board,
            Piece::from_letter('B').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('b').unwrap(),
            move_piece,
            all_moves,
        );
    }


    #[test]
    fn all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('B').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('b').unwrap(),
            move_piece,
            all_moves,
        );
    }


    #[test]
    fn valid_moves() {
        let board = test_board();
        // - Straight
        assert!(move_piece(&board, &c(6, 1), &m(5, 1)).is_err());
        // - Diagonal
        move_piece(&board, &c(6, 1), &m(7, 2)).unwrap();
        move_piece(&board, &c(6, 1), &m(5, 0)).unwrap();

        let turned = board.turned();
        // - Straight
        assert!(move_piece(&turned, &c(2, 2), &m(7, 2)).is_err());
        // - Diagonal
        move_piece(&turned, &c(2, 2), &m(5, 5)).unwrap();
        move_piece(&turned, &c(2, 2), &m(4, 0)).unwrap();
    }


    #[test]
    fn valid_captures() {
        let board = test_board();
        move_piece(&board, &c(6, 1), &m(4, 3)).unwrap();

        let turned = board.turned();
        move_piece(&turned, &c(2, 2), &m(6, 6)).unwrap();
    }
}
