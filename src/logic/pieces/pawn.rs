use crate::logic::basic::{Coordinate, Player};
use crate::logic::board::{Board, TileContent, BOARD_SIZE};
use crate::utils::ValueError;

use super::MoveError;


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Coordinate> {
    let mut moves = Vec::new();

    if let Ok(to) = coordinate_up(&board.turn, from, 1) {
        if matches!(board.get_tile(&to), TileContent::Empty) {
            // - Regular move
            moves.push(to.clone());

            // - Double move
            if is_player_pawn_original_pos(board, from) {
                if let Ok(to_double) = coordinate_up(&board.turn, from, 2) {
                    if matches!(board.get_tile(&to), TileContent::Empty) {
                        moves.push(to_double);
                    }
                }
            }
        }

        let to_x = to.xv();
        let to_y = to.yv();

        if to_x > 0 {
            let to_left = Coordinate::try_new(to_x - 1, to_y).unwrap();

            // - Regular capture
            if let TileContent::Piece(piece) = board.get_tile(&to_left) {
                if piece.player != board.turn {
                    moves.push(to_left);
                }
            }
            // - En Passant
            else if is_en_passant(board, &to_left) {
                moves.push(to_left);
            }
        }

        if let Ok(to_right) = Coordinate::try_new(to_x + 1, to_y){
            // - Regular capture
            if let TileContent::Piece(piece) = board.get_tile(&to_right) {
                if piece.player != board.turn {
                    moves.push(to_right);
                }
            }
            // - En Passant
            else if is_en_passant(board, &to_right) {
                moves.push(to_right);
            }
        }
    }

    // TODO: Promote

    moves
}


pub fn move_piece(board: &Board, from: &Coordinate, to: &Coordinate) -> Result<Board, MoveError> {
    // Piece at `from` and `piece` is from player with turn already checked
    let from_x = from.xv();
    let to_x = to.xv();

    // Move
    if from_x == to_x {
        if !matches!(board.get_tile(to), TileContent::Empty) {
            return Err(MoveError::IllegalMove);
        }

        // - Regular move
        let regular_move = coordinate_up(&board.turn, from, 1);

        if let Ok(coord_up) = regular_move {
            if coord_up == *to {
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                return Ok(new_board);
            }

            // - Double move
            let is_double_move =
                // On original position
                is_player_pawn_original_pos(board, from)
                // No piece in between
                && matches!(board.get_tile(&coord_up), TileContent::Empty)
                // Move is actually a double move
                && coordinate_up(
                    &board.turn, from, 2
                ).map_or(false, |coord| &coord == to);

            if is_double_move {
                let mut new_board = board.turned();
                new_board.en_passant = Some(to.clone());
                new_board.move_tile(from, to);
                return Ok(new_board);
            }
        }

        return Err(MoveError::IllegalMove);
    }

    // Capture
    if is_move_up_diagonal(&board.turn, from, to) {
        return match board.get_tile(to) {
            // - Regular capture
            TileContent::Piece(piece) => {
                if piece.player != board.turn {
                    let mut new_board = board.turned();
                    new_board.move_tile(from, to);
                    Ok(new_board)
                } else {
                    Err(MoveError::IllegalMove)
                }
            },
            // - En Passant
            TileContent::Empty => {
                if is_en_passant(board, to) {
                    let mut new_board = board.turned();
                    new_board.clear_tile(board.en_passant.as_ref().unwrap());
                    new_board.move_tile(from, to);
                    Ok(new_board)
                } else {
                    Err(MoveError::IllegalMove)
                }
            },
        }
    }

    // TODO: Promote

    Err(MoveError::IllegalMove)
}


fn is_player_pawn_original_pos(board: &Board, coordinate: &Coordinate) -> bool {
    (match board.turn {
        Player::White => BOARD_SIZE - 2,
        Player::Black => 1,
    } == coordinate.yv())
}


fn coordinate_up(player: &Player, from: &Coordinate, steps: usize) -> Result<Coordinate, ValueError> {
    let from_x = from.xv();
    let from_y = from.yv();

    match player {
        Player::White if steps <= from_y => Coordinate::try_new(from_x, from_y - steps),
        Player::Black => Coordinate::try_new(from_x, from_y + steps),
        _ => Err(ValueError),
    }
}


fn is_en_passant(board: &Board, to: &Coordinate) -> bool {
    board
        .en_passant
        .as_ref()
        .and_then(|coord| coordinate_up(&board.turn, coord, 1).ok())
        .map_or(false, |target| target == *to)
}


fn is_move_up_diagonal(player: &Player, from: &Coordinate, to: &Coordinate) -> bool {
    match coordinate_up(player, from, 1) {
        Ok(up) => up.yv() == to.yv() && from.xv().abs_diff(to.xv()) == 1,
        Err(ValueError) => false,
    }
}


#[cfg(test)]
mod tests {
    use crate::logic::board::Board;
    use crate::logic::pieces::tests::{c, assert_all_moves_valid, assert_valid_in_all_moves};

    use super::{move_piece, all_moves};


    fn test_board() -> Board {
        Board::from_configuration([
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            [' ', ' ', 'k', ' ', ' ', 'p', ' ', ' '],
            [' ', ' ', ' ', 'K', ' ', ' ', ' ', 'p'],
            [' ', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ])
    }


    #[test]
    fn all_moves_are_valid() {
        let board = test_board();
        assert_all_moves_valid(&board, move_piece, all_moves);

        let turned = board.turned();
        assert_all_moves_valid(&turned, move_piece, all_moves);
    }


    #[test]
    fn all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(&board, move_piece, all_moves);

        let turned = board.turned();
        assert_valid_in_all_moves(&turned, move_piece, all_moves);
    }

    
    #[test]
    fn valid_regular_moves() {
        let board = test_board();

        move_piece(&board, &c(0, 3), &c(0, 2)).unwrap();
        move_piece(&board, &c(2, 6), &c(2, 5)).unwrap();
        // Cannot return
        assert!(move_piece(&board, &c(0, 3), &c(0, 4)).is_err()); 
        // Cannot move to the side
        assert!(move_piece(&board, &c(1, 6), &c(0, 6)).is_err());
        // Cannot move through figures
        assert!(move_piece(&board, &c(3, 6), &c(3, 5)).is_err());
        assert!(move_piece(&board, &c(7, 6), &c(7, 5)).is_err());

        let turned = board.turned();
        move_piece(&turned, &c(5, 4), &c(5, 5)).unwrap();
        // Cannot return
        assert!(move_piece(&turned, &c(5, 4), &c(5, 3)).is_err());
        // Cannot move through figures
        assert!(move_piece(&turned, &c(7, 5), &c(7, 6)).is_err());
    }


    #[test]
    fn valid_double_moves() {
        let board = test_board();

        move_piece(&board, &c(1, 6), &c(1, 4)).unwrap();
        assert!(move_piece(&board, &c(0, 3), &c(0, 1)).is_err());

        let turned = board.turned();
        move_piece(&turned, &c(1, 1), &c(1, 3)).unwrap();
        assert!(move_piece(&turned, &c(4, 3), &c(4, 5)).is_err());
    }


    #[test]
    fn valid_regular_caputres() {
        let board = test_board();

        move_piece(&board, &c(6, 6), &c(7, 5)).unwrap();
        // Cannot throw own
        assert!(move_piece(&board, &c(2, 6), &c(3, 5)).is_err());
        // Cannot throw outside of diagonal
        assert!(move_piece(&board, &c(1, 1), &c(2, 4)).is_err());
    }


    // TODO: En passant
    // TODO: Promote
}
