use crate::logic::basic::Coordinate;
use crate::logic::board::{Board, BOARD_SIZE};

use super::{Move, MoveError, is_friendly_fire};


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    let (x, y) = from.values();
    let mut moves = Vec::with_capacity(8);

    for delta_x in -1..=1 {
        for delta_y in -1..=1 {
            if delta_x == 0 && delta_y == 0 {
                continue;
            }

            let to_x = x as i16 + delta_x;
            let to_y = y as i16 + delta_y;

            if to_x < 0 || to_x >= BOARD_SIZE as i16 || to_y < 0 || to_y >= BOARD_SIZE as i16 {
                continue;
            }

            let coord = Coordinate::try_new(to_x as usize, to_y as usize).unwrap();
            if !is_friendly_fire(board, &coord) {
                moves.push(Move::Regular(coord));
            }
        }
    }

    moves
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

            let (from_x, from_y) = from.values();
            let (to_x, to_y) = to.values();

            let delta_x = from_x.abs_diff(to_x);
            let delta_y = from_y.abs_diff(to_y);

            if delta_x < 2 && delta_y < 2 {
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
    use crate::logic::pieces::tests::{assert_all_moves_valid, assert_valid_in_all_moves};

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
    fn all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('K').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('k').unwrap(),
            move_piece,
            all_moves,
        );
    }
}
