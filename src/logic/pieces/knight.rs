use crate::logic::basic::Coordinate;
use crate::logic::board::{Board, TileContent, BOARD_SIZE};

use super::{Move, MoveError};

pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    let (x, y) = from.values();
    let mut moves = Vec::with_capacity(8);

    for delta_x in 1..=2 {
        let delta_y = 3 - delta_x;

        if x + delta_x < BOARD_SIZE && y + delta_y < BOARD_SIZE {
            let coord = Coordinate::try_new(x + delta_x, y + delta_y).unwrap();
            if !is_friendly_fire(board, &coord) {
                moves.push(Move::Regular(coord));
            }
        }
        if x >= delta_x && y + delta_y < BOARD_SIZE {
            let coord = Coordinate::try_new(x - delta_x, y + delta_y).unwrap();
            if !is_friendly_fire(board, &coord) {
                moves.push(Move::Regular(coord));
            }
        }
        if x + delta_x < BOARD_SIZE && y >= delta_y {
            let coord = Coordinate::try_new(x + delta_x, y - delta_y).unwrap();
            if !is_friendly_fire(board, &coord) {
                moves.push(Move::Regular(coord));
            }
        }
        if x >= delta_x && y >= delta_y {
            let coord = Coordinate::try_new(x - delta_x, y - delta_y).unwrap();
            if !is_friendly_fire(board, &coord) {
                moves.push(Move::Regular(coord));
            }
        }
    }

    moves
}

fn is_friendly_fire(board: &Board, coordinate: &Coordinate) -> bool {
    match board.get_tile(coordinate) {
        TileContent::Piece(piece) => piece.player == board.turn,
        TileContent::Empty => false,
    }
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

            let (from_x, from_y) = from.values();
            let (to_x, to_y) = to.values();

            let delta_x = from_x.abs_diff(to_x);
            let delta_y = from_y.abs_diff(to_y);

            if delta_x == 2 && delta_y == 1 || delta_x == 1 && delta_y == 2 {
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
    use crate::logic::pieces::tests::{assert_all_moves_valid, assert_valid_in_all_moves, c, m};
    use crate::logic::pieces::Piece;

    use super::{all_moves, move_piece};

    fn test_board() -> Board {
        Board::from_configuration([
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', 'P', ' ', 'B', ' '],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            [' ', ' ', 'n', ' ', ' ', 'p', ' ', ' '],
            [' ', ' ', ' ', 'N', ' ', ' ', ' ', 'p'],
            [' ', 'P', 'P', 'P', ' ', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ])
    }

    #[test]
    fn test_all_moves_are_valid() {
        let board = test_board();
        assert_all_moves_valid(
            &board,
            Piece::from_letter('N').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('n').unwrap(),
            move_piece,
            all_moves,
        );
    }

    #[test]
    fn test_all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('N').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('n').unwrap(),
            move_piece,
            all_moves,
        );
    }

    #[test]
    fn test_moves() {
        let board = test_board();
        move_piece(&board, &c(3, 5), &m(2, 3)).unwrap();
        assert!(move_piece(&board, &c(3, 5), &m(1, 6)).is_err());
        move_piece(&board, &c(3, 5), &m(1, 4)).unwrap();
        move_piece(&board, &c(3, 5), &m(2, 7)).unwrap();
        move_piece(&board, &c(3, 5), &m(4, 7)).unwrap();
        assert!(move_piece(&board, &c(3, 5), &m(1, 7)).is_err());
    }

    #[test]
    fn test_captures() {
        let board = test_board();
        move_piece(&board, &c(3, 5), &m(4, 3)).unwrap();
        assert!(move_piece(&board, &c(3, 5), &m(5, 6)).is_err());
    }
}
