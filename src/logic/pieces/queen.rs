use crate::logic::basic::Coordinate;
use crate::logic::board::{Board, TileContent, BOARD_SIZE};

use super::{Move, MoveError};


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    let mut moves = all_moves_straight(board, from);
    moves.append(&mut all_moves_diagonal(board, from));
    moves
}


pub fn all_moves_straight(board: &Board, from: &Coordinate) -> Vec<Move> {
    let mut moves = Vec::new();

    let (from_x, from_y) = from.values();

    // Down
    for y in from_y + 1..BOARD_SIZE {
        let coord = Coordinate::try_new(from_x, y).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Up
    for y in (0..from_y).rev() {
        let coord = Coordinate::try_new(from_x, y).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Right
    for x in from_x + 1..BOARD_SIZE {
        let coord = Coordinate::try_new(x, from_y).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Left
    for x in (0..from_x).rev() {
        let coord = Coordinate::try_new(x, from_y).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    moves
}


pub fn all_moves_diagonal(board: &Board, from: &Coordinate) -> Vec<Move> {
       let mut moves = Vec::new();

    let (from_x, from_y) = from.values();

    // Down-right
    for i in 1..(BOARD_SIZE - from_x).min(BOARD_SIZE - from_y) {
        let coord = Coordinate::try_new(from_x + i, from_y + i).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Up-left
    for i in 1..=from_x.min(from_y) {
        let coord = Coordinate::try_new(from_x - i, from_y - i).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Down-left
    for i in 1..=from_x.min(BOARD_SIZE - from_y - 1) {
        let coord = Coordinate::try_new(from_x - i, from_y + i).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
        }
    }

    // Up-right
    for i in 1..=from_y.min(BOARD_SIZE - from_x - 1) {
        let coord = Coordinate::try_new(from_x + i, from_y - i).unwrap();
        if let TileContent::Piece(piece) = board.get_tile(&coord) {
            if piece.player != board.turn {
                moves.push(Move::Regular(coord));
            }
            break;
        } else {
            moves.push(Move::Regular(coord));
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

            if let TileContent::Piece(piece) = board.get_tile(to) {
                if piece.player == board.turn {
                    return Err(MoveError::IllegalMove);
                }
            }

            if is_straight(from, to) {
                // - Straight
                if piece_between_straight(board, from, to) {
                    return Err(MoveError::IllegalMove);
                }
            } else if is_diagonal(from, to) {
                // -- Diagonal
                if piece_between_diagonal(board, from, to) {
                    return Err(MoveError::IllegalMove);
                }
            } else {
                return Err(MoveError::IllegalMove);
            }

            let mut new_board = board.turned();
            new_board.move_tile(from, to);
            Ok(new_board)
        },
    };
}


pub fn is_straight(from: &Coordinate, to: &Coordinate) -> bool {
    let (from_x, from_y) = from.values();
    let (to_x, to_y) = to.values();
    from_x == to_x || from_y == to_y
}


pub fn is_diagonal(from: &Coordinate, to: &Coordinate) -> bool {
    let (from_x, from_y) = from.values();
    let (to_x, to_y) = to.values();
    from_x.abs_diff(to_x) == from_y.abs_diff(to_y)
}


pub fn piece_between_straight(board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
    let (from_x, from_y) = from.values();
    let (to_x, to_y) = to.values();

    if from_x == to_x {
        for y in (from_y.min(to_y) + 1)..(from_y.max(to_y)) {
            let coord = Coordinate::try_new(from_x, y).unwrap();
            
            if !matches!(board.get_tile(&coord), TileContent::Empty) {
                return true;
            }
        }
    } else {
        for x in (from_x.min(to_x) + 1)..(from_x.max(to_x)) {
            let coord = Coordinate::try_new(x, from_y).unwrap();

            if !matches!(board.get_tile(&coord), TileContent::Empty) {
                return true;
            }
        }
    }
    
    false
}


pub fn piece_between_diagonal(board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
    let (from_x, from_y) = from.values();
    let (to_x, to_y) = to.values();

    let steps = from_x.abs_diff(to_x);
    let delta_x: i16 = if from_x > to_x { -1 } else { 1 };
    let delta_y: i16 = if from_y > to_y { -1 } else { 1 };

    for i in 1..steps {
        let coord = Coordinate::try_new(
            (from_x as i16 + (i as i16 * delta_x)) as usize,
            (from_y as i16 + (i as i16 * delta_y)) as usize,
        ).unwrap();

        if !matches!(board.get_tile(&coord), TileContent::Empty) {
            return true;
        }
    }

    false
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
            [' ', 'p', ' ', ' ', 'P', ' ', 'Q', ' '],
            [' ', ' ', 'q', ' ', ' ', ' ', ' ', ' '],
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
            Piece::from_letter('Q').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('q').unwrap(),
            move_piece,
            all_moves,
        );
    }


    #[test]
    fn all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('Q').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('q').unwrap(),
            move_piece,
            all_moves,
        );
    }

    #[test]
    fn valid_moves() {
        let board = test_board();
        // - Straight
        move_piece(&board, &c(6, 1), &m(5, 1)).unwrap();
        // Own piece left
        assert!(move_piece(&board, &c(6, 1), &m(4, 1)).is_err());
        // Past piece left
        assert!(move_piece(&board, &c(6, 1), &m(3, 1)).is_err());

        // - Diagonal
        move_piece(&board, &c(6, 1), &m(7, 2)).unwrap();
        move_piece(&board, &c(6, 1), &m(5, 0)).unwrap();

        let turned = board.turned();
        // - Straight
        move_piece(&turned, &c(2, 2), &m(7, 2)).unwrap();
        // Past piece down
        assert!(move_piece(&board, &c(2, 2), &m(2, 5)).is_err());
        // - Diagonal
        move_piece(&turned, &c(2, 2), &m(5, 5)).unwrap();
        move_piece(&turned, &c(2, 2), &m(4, 0)).unwrap();
    }

    #[test]
    fn valid_captures() {
        let board = test_board();
        // - Straight
        // Own piece
        assert!(move_piece(&board, &c(6, 1), &m(6, 6)).is_err());
        // - Diagonal
        move_piece(&board, &c(6, 1), &m(4, 3)).unwrap();

        let turned = board.turned();
        // - Diagonal
        move_piece(&turned, &c(2, 2), &m(6, 6)).unwrap();
    }
}
