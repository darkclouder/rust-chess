use std::cmp;
use crate::logic::basic::{Coordinate, Player};
use crate::logic::board::{Board, TileContent};
use crate::utils::ValueError;

use super::MoveError;


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Coordinate> {
    let mut moves = Vec::new();

    if let Ok(to) = move_up(&board.turn, from, 1) {
        if matches!(board.get_tile(&to), TileContent::Empty) {
            // - Regular move
            moves.push(to.clone());

            // - Double move
            if let Ok(to) = move_up(&board.turn, from, 2) {
                // TODO: Forgot to only do it on first move
                if matches!(board.get_tile(&to), TileContent::Empty) {
                    moves.push(to.clone());
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
        let regular_move = move_up(&board.turn, from, 1);

        if let Ok(coord) = regular_move {
            if coord == *to {
                let mut new_board = board.turned();
                new_board.move_tile(from, to);
                return Ok(new_board);
            }

            if matches!(board.get_tile(&coord), TileContent::Empty) {
                // - Double move
                if move_up(&board.turn, from, 2).map_or(false, |coord| &coord == to) {
                    // TODO: Forgot to only do it on first move
                    let mut new_board = board.turned();
                    new_board.en_passant = Some(to.clone());
                    new_board.move_tile(from, to);
                    return Ok(new_board);
                }
            }
        }

        return Err(MoveError);
    }

    // Capture
    if cmp::max(from_x, to_x) - cmp::min(from_x, to_x) == 1 {
       return match board.get_tile(to) {
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
        }
    }

    // TODO: Promote

    Err(MoveError)
}


fn move_up(player: &Player, from: &Coordinate, steps: usize) -> Result<Coordinate, ValueError> {
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
        .and_then(|coord| move_up(&board.turn, &coord, 1).ok())
}
