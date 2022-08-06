use std::cmp;
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
                if let Ok(to) = coordinate_up(&board.turn, from, 2) {
                    if matches!(board.get_tile(&to), TileContent::Empty) {
                        moves.push(to.clone());
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
            else if en_passant_to(board).map_or(false, |target| target == to_left) {
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
            else if en_passant_to(board).map_or(false, |target| target == to_right) {
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
            return Err(MoveError);
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

        return Err(MoveError);
    }

    // Capture
    if cmp::max(from_x, to_x) - cmp::min(from_x, to_x) == 1 {
       return match board.get_tile(to) {
            // - Regular capture
            TileContent::Piece(piece) => {
                if piece.player != board.turn {
                    let mut new_board = board.turned();
                    new_board.move_tile(from, to);
                    Ok(new_board)
                } else {
                    Err(MoveError)
                }
            },
            // - En Passant
            TileContent::Empty => {
                if en_passant_to(board).map_or(false, |target| target == *to) {
                    let mut new_board = board.turned();
                    new_board.clear_tile(board.en_passant.as_ref().unwrap());
                    new_board.move_tile(from, to);
                    Ok(new_board)
                } else {
                    Err(MoveError)
                }
            },
        }
    }

    // TODO: Promote

    Err(MoveError)
}


fn is_player_pawn_original_pos(board: &Board, coordinate: &Coordinate) -> bool {
    return match board.turn {
        Player::White => BOARD_SIZE - 2,
        Player::Black => 1,
    } == coordinate.yv();
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


fn en_passant_to(board: &Board) -> Option<Coordinate> {
    board
        .en_passant
        .as_ref()
        .and_then(|coord| coordinate_up(&board.turn, &coord, 1).ok())
}


#[cfg(test)]
mod tests {
    use crate::logic::basic::Coordinate;
    use crate::logic::board::Board;

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

    fn assert_all_moves_valid(board: &Board, from: &Coordinate, moves: &Vec<Coordinate>) {
        for to in moves {
            match move_piece(board, from, to) {
                Err(e) => assert!(
                    false,
                    "Could not move from {} to {} as {:?}: {}",
                    from, to, board.turn, e
                ),
                Ok(_) => (),
            };
        }
    }

    fn c(x: usize, y: usize) -> Coordinate {
        Coordinate::try_new(x, y).unwrap()
    }

    #[test]
    fn all_moves_are_valid() {
        let board = test_board();

        for x in 0..8 {
            for y in 0..8 {
                let from = c(x, y);
                assert_all_moves_valid(&board, &from, &all_moves(&board, &from));
            }
        }

        let turned = board.turned();
        for x in 0..8 {
            for y in 0..8 {
                let from = c(x, y);
                assert_all_moves_valid(&turned, &from, &all_moves(&turned, &from));
            }
        }
    }

    #[test]
    fn valid_regular_moves() {
        let board = test_board();

        move_piece(&board, &c(0, 3), &c(0, 2)).unwrap();
        move_piece(&board, &c(2, 6), &c(2, 5)).unwrap();
        // Cannot return
        assert!(move_piece(&board, &c(0, 3), &c(0, 4)).is_err()); 
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

    // TODO: Regular capture
    // TODO: En passant
    // TODO: Promote
}
