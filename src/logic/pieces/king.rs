use crate::logic::basic::Coordinate;
use crate::logic::board::{Board, TileContent, BOARD_SIZE};

use super::queen::piece_between_straight;
use super::{is_friendly_fire, Move, MoveError, PieceType};

pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    let (x, y) = from.values();
    let mut moves = Vec::with_capacity(8);

    // - Regular
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

    // -- Castling
    if x + 2 < BOARD_SIZE {
        let to = Coordinate::try_new(x + 2, y).unwrap();
        if get_castling_rook(board, from, &to).is_some() {
            moves.push(Move::Regular(to));
        }
    }
    if x >= 2 {
        let to = Coordinate::try_new(x - 2, y).unwrap();
        if get_castling_rook(board, from, &to).is_some() {
            moves.push(Move::Regular(to));
        }
    }

    moves
}

pub fn move_piece(board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
    match a_move {
        Move::Promotion(..) => Err(MoveError::IllegalMove),
        Move::Regular(to) => {
            if from == to {
                return Err(MoveError::IllegalMove);
            }

            let (from_x, from_y) = from.values();
            let (to_x, to_y) = to.values();

            let delta_x = from_x.abs_diff(to_x);
            let delta_y = from_y.abs_diff(to_y);

            if delta_x < 2 && delta_y < 2 {
                if is_friendly_fire(board, to) {
                    return Err(MoveError::IllegalMove);
                }

                // - Regular move
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                Ok(new_board)
            } else if let Some(rook_coord) = get_castling_rook(board, from, to) {
                // - Castling
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                new_board.move_tile(&rook_coord, from);
                Ok(new_board)
            } else {
                Err(MoveError::IllegalMove)
            }
        }
    }
}

fn get_castling_rook(board: &Board, from: &Coordinate, to: &Coordinate) -> Option<Coordinate> {
    let (from_x, from_y) = from.values();
    let (to_x, to_y) = to.values();
    let delta_x = from_x.abs_diff(to_x);
    let delta_y = from_y.abs_diff(to_y);

    if delta_y != 0 || delta_x != 2 {
        return None;
    }

    if let TileContent::Piece(piece) = board.get_tile(from) {
        if piece.moved {
            return None;
        }
    } else {
        return None;
    }

    let rook_x = if from_x < to_x { BOARD_SIZE - 1 } else { 0 };
    let rook_coord = Coordinate::try_new(rook_x, from_y).unwrap();

    if let TileContent::Piece(piece) = board.get_tile(&rook_coord) {
        if piece.moved {
            return None;
        }
        if !matches!(piece.piece_type, PieceType::Rook) {
            return None;
        }
        if piece.player != board.turn {
            return None;
        }
        if piece_between_straight(board, from, &rook_coord) {
            return None;
        }
        Some(rook_coord)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::board::{Board, TileContent};
    use crate::logic::pieces::tests::{assert_all_moves_valid, assert_valid_in_all_moves, c, m};
    use crate::logic::pieces::{Piece, PieceType};

    use super::{all_moves, move_piece};

    fn test_board() -> Board {
        Board::from_configuration([
            ['r', ' ', ' ', 'k', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', 'P', ' ', 'B', ' '],
            [' ', ' ', 'b', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', 'p', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'p'],
            [' ', 'P', 'P', 'P', ' ', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', 'K', ' ', ' ', 'R'],
        ])
    }

    #[test]
    fn test_all_moves_are_valid() {
        let board = test_board();
        assert_all_moves_valid(
            &board,
            Piece::from_letter('K').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('k').unwrap(),
            move_piece,
            all_moves,
        );
    }

    #[test]
    fn test_all_valid_are_moves() {
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

    #[test]
    fn test_castling() {
        let board = test_board();
        {
            let new_board = move_piece(&board, &c(4, 7), &m(6, 7)).unwrap();
            assert!(matches!(
                new_board.get_tile(&c(6, 7)),
                TileContent::Piece(_)
            ));
            if let TileContent::Piece(piece) = new_board.get_tile(&c(6, 7)) {
                assert!(matches!(piece.piece_type, PieceType::King));
                assert!(piece.moved);
            }
            assert!(matches!(
                new_board.get_tile(&c(4, 7)),
                TileContent::Piece(_)
            ));
            if let TileContent::Piece(piece) = new_board.get_tile(&c(4, 7)) {
                assert!(matches!(piece.piece_type, PieceType::Rook));
                assert!(piece.moved);
            }
        }

        // This was a faulty result. It makes no sense at all.
        // Just keeping it here to assure it does not come back
        assert!(move_piece(&board, &c(4, 7), &m(6, 0)).is_err());
    }
}
