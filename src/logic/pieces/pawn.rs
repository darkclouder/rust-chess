use crate::logic::basic::{Coordinate, Player};
use crate::logic::board::{Board, TileContent, BOARD_SIZE};
use crate::utils::ValueError;

use super::{MoveError, Move, PieceType};


pub fn all_moves(board: &Board, from: &Coordinate) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let promotion_y = match board.turn {
        Player::White => 0,
        Player::Black => BOARD_SIZE - 1,
    };

    for to in all_moves_regular(board, from) {
        if promotion_y == to.yv() {
            // Requires promotion
            moves.push(Move::Promotion(to.clone(), PieceType::Queen));
            moves.push(Move::Promotion(to.clone(), PieceType::Rook));
            moves.push(Move::Promotion(to.clone(), PieceType::Knight));
            moves.push(Move::Promotion(to, PieceType::Bishop));
        } else {
            // Regular move
            moves.push(Move::Regular(to));
        }
    }

    moves
}


fn all_moves_regular(board: &Board, from: &Coordinate) -> Vec<Coordinate> {
    let mut moves: Vec<Coordinate> = Vec::new();

    if let Ok(to) = coordinate_up(&board.turn, from, 1) {
        if matches!(board.get_tile(&to), TileContent::Empty) {
            // - Regular move
            moves.push(to.clone());

            // - Double move
            if is_player_pawn_original_pos(board, from) {
                if let Ok(to_double) = coordinate_up(&board.turn, from, 2) {
                    if matches!(board.get_tile(&to_double), TileContent::Empty) {
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

    moves
}


pub fn move_piece(board: &Board, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
    if let Move::Promotion(_, new_type) = a_move {
        if !matches!(new_type, PieceType::Queen | PieceType::Rook | PieceType::Knight | PieceType::Bishop) {
            return Err(MoveError::IllegalMove);
        }
    }

    let to = a_move.get_to();
    let mut new_board = move_piece_regular(board, from, to)?;

    let requires_promotion = match board.turn {
        Player::White => 0,
        Player::Black => BOARD_SIZE - 1,
    } == to.yv();

    match a_move {
        Move::Regular(_) if requires_promotion => Err(MoveError::PromotionRequired),
        Move::Regular(_) if !requires_promotion => Ok(new_board),
        Move::Promotion(_, new_type) if requires_promotion => {
            if let TileContent::Piece(piece) = new_board.get_tile(to) {
                let new_tile = TileContent::Piece(piece.promoted(new_type.clone()));
                new_board.set_tile(to, new_tile);
                Ok(new_board)
            } else {
                panic!("Illegal state at move {:?} of player {:?} from {}", a_move, board.turn, from);
            }
        },
        _ => Err(MoveError::IllegalMove),
    }
}


fn move_piece_regular(board: &Board, from: &Coordinate, to: &Coordinate) -> Result<Board, MoveError> {
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
                    let en_passant = board.en_passant.as_ref().unwrap();
                    new_board.clear_tile(en_passant);
                    new_board.move_tile(from, to);
                    Ok(new_board)
                } else {
                    Err(MoveError::IllegalMove)
                }
            },
        }
    }

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
    use crate::logic::basic::Player;
    use crate::logic::board::{Board, TileContent};
    use crate::logic::pieces::{PieceType, Move, Piece};
    use crate::logic::pieces::tests::{c, m, assert_all_moves_valid, assert_valid_in_all_moves};

    use super::{move_piece, all_moves};


    fn test_board() -> Board {
        Board::from_configuration([
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'p', ' ', ' ', 'P', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
            [' ', ' ', 'k', ' ', ' ', 'p', ' ', ' '],
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
            Piece::from_letter('P').unwrap(),
            move_piece,
            all_moves,
        );

        let turned = board.turned();
        assert_all_moves_valid(
            &turned,
            Piece::from_letter('p').unwrap(),
            move_piece,
            all_moves,
        );
    }


    #[test]
    fn test_all_valid_are_moves() {
        let board = test_board();
        assert_valid_in_all_moves(
            &board,
            Piece::from_letter('P').unwrap(),
            move_piece,
            all_moves,
        );
        // En passant
        {
            let prepared = move_piece(&board, &c(6, 6), &m(6, 4)).unwrap();
            assert_valid_in_all_moves(
                &prepared,
                Piece::from_letter('P').unwrap(),
                move_piece,
                all_moves,
            );
        }

        let turned = board.turned();
        assert_valid_in_all_moves(
            &turned,
            Piece::from_letter('p').unwrap(),
            move_piece,
            all_moves,
        );
        // En passant
        {
            let prepared = move_piece(&turned, &c(1, 1), &m(1, 3)).unwrap();
            assert_valid_in_all_moves(
                &prepared,
                Piece::from_letter('p').unwrap(),
                move_piece,
                all_moves,
            );
        }
    }

    
    #[test]
    fn test_regular_moves() {
        let board = test_board();

        move_piece(&board, &c(0, 3), &m(0, 2)).unwrap();
        move_piece(&board, &c(2, 6), &m(2, 5)).unwrap();
        // Cannot return
        assert!(move_piece(&board, &c(0, 3), &m(0, 4)).is_err());
        // Cannot move to the side
        assert!(move_piece(&board, &c(1, 6), &m(0, 6)).is_err());
        // Cannot move through figures
        assert!(move_piece(&board, &c(3, 6), &m(3, 5)).is_err());
        assert!(move_piece(&board, &c(7, 6), &m(7, 5)).is_err());

        let turned = board.turned();
        move_piece(&turned, &c(5, 4), &m(5, 5)).unwrap();
        // Cannot return
        assert!(move_piece(&turned, &c(5, 4), &m(5, 3)).is_err());
        // Cannot move through figures
        assert!(move_piece(&turned, &c(7, 5), &m(7, 6)).is_err());
    }


    #[test]
    fn test_double_moves() {
        let board = test_board();

        move_piece(&board, &c(1, 6), &m(1, 4)).unwrap();
        assert!(move_piece(&board, &c(0, 3), &m(0, 1)).is_err());

        let turned = board.turned();
        move_piece(&turned, &c(1, 1), &m(1, 3)).unwrap();
        assert!(move_piece(&turned, &c(4, 3), &m(4, 5)).is_err());
    }


    #[test]
    fn test_regular_captures() {
        let board = test_board();

        let new_board = move_piece(&board, &c(6, 6), &m(7, 5)).unwrap();
        assert!(matches!(new_board.get_tile(&c(6, 6)), TileContent::Empty));
        assert!(matches!(new_board.get_tile(&c(7, 5)), TileContent::Piece(_)));
        if let TileContent::Piece(piece) = new_board.get_tile(&c(7, 5)) {
            assert!(matches!(piece.piece_type, PieceType::Pawn));
            assert!(matches!(piece.player, Player::White));
        }

        // Cannot throw own
        assert!(move_piece(&board, &c(2, 6), &m(3, 5)).is_err());
        // Cannot throw outside of diagonal
        assert!(move_piece(&board, &c(1, 1), &m(2, 4)).is_err());
    }

    #[test]
    fn test_en_passants() {
        let board = test_board();
        {
            let prepared = move_piece(&board, &c(6, 6), &m(6, 4)).unwrap();
            assert!(!matches!(prepared.get_tile(&c(6, 4)), TileContent::Empty));
            let new_board = move_piece(&prepared, &c(5, 4), &m(6, 5)).unwrap();
            assert!(matches!(new_board.get_tile(&c(6, 4)), TileContent::Empty));
        }
        assert!(move_piece(&board, &c(0, 3), &m(1, 2)).is_err());

        let turned = board.turned();
        {
            let prepared = move_piece(&turned, &c(1, 1), &m(1, 3)).unwrap();
            assert!(!matches!(prepared.get_tile(&c(1, 3)), TileContent::Empty));
            let new_board = move_piece(&prepared, &c(0, 3), &m(1, 2)).unwrap();
            assert!(matches!(new_board.get_tile(&c(1, 3)), TileContent::Empty));
        }
        assert!(move_piece(&turned, &c(5, 4), &m(6, 5)).is_err());
    }


    #[test]
    fn test_promotion() {
        let board = test_board();

        assert!(move_piece(&board, &c(4, 1), &m(4, 0)).is_err());

        assert!(move_piece(
            &board,
            &c(4, 1),
            &Move::Promotion(c(4, 0), PieceType::King)
        ).is_err());

        {
            let result = move_piece(
                &board,
                &c(4, 1),
                &Move::Promotion(c(4, 0), PieceType::Queen)
            ).unwrap();

            if let TileContent::Piece(piece) = result.get_tile(&c(4, 0)) {
                assert!(matches!(piece.piece_type, PieceType::Queen));
            } else {
                panic!("Promotion did not work");
            }
        }
    }
}
