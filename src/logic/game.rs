use crate::logic::basic::{Coordinate};
use crate::logic::board::{Board, TileContent};

use super::pieces::{MoveError, Move, PieceType};


pub enum GameState {
    WaitMove(bool),
    SelectPromotionType(Coordinate, Coordinate),
    CheckMate,
}


pub struct Game {
    pub board: Board,
    pub state: GameState,
}


impl Game {
    pub fn default() -> Self {
        let board = Board::default();
        let is_check = board.is_player_on_check(&board.turn);

        Self {
            board,
            state: GameState::WaitMove(is_check),
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::default();
        let is_check = self.board.is_player_on_check(&self.board.turn);
        self.state = GameState::WaitMove(is_check);
    }

    pub fn can_move_from(&self, coordinate: &Coordinate) -> bool {
        let tile = self.board.get_tile(coordinate);
        
        match tile {
            TileContent::Empty => false,
            TileContent::Piece(piece) => piece.player == self.board.turn,
        }
    }

    pub fn can_move(&self, from: &Coordinate, to: &Coordinate) -> bool {
        let tile = self.board.get_tile(from);
        
        match tile {
            TileContent::Piece(piece) => piece.can_move( &self.board, from, to),
            TileContent::Empty => false,
        }
    }

    pub fn move_piece_with_promotion(
        &mut self,
        from: &Coordinate,
        to: &Coordinate,
        piece_type: &PieceType,
    ) -> Result<(), MoveError> {
        let moved = self.try_move(from, &Move::Promotion(to.clone(), piece_type.clone()));

        match moved {
            Ok(new_board) => {
                self.board = new_board;
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        let moved = self.try_move(from, &Move::Regular(to.clone()));

        match moved {
            Ok(new_board) => {
                self.board = new_board;
                Ok(())
            },
            Err(MoveError::PromotionRequired) => {
                self.state = GameState::SelectPromotionType(from.clone(), to.clone());
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    fn try_move(&mut self, from: &Coordinate, a_move: &Move) -> Result<Board, MoveError> {
        let tile = self.board.get_tile(from);

        let new_board = match tile {
            TileContent::Piece(piece) => piece.move_piece(&self.board, from, a_move),
            TileContent::Empty => Err(MoveError::IllegalMove),
        }?;

        if new_board.is_player_on_check(&self.board.turn) {
            Err(MoveError::IsCheck)
        } else {
            let on_check = new_board.is_player_on_check(&new_board.turn);

            if on_check && new_board.is_current_player_checkmate() {
                self.state = GameState::CheckMate;
            } else {
                self.state = GameState::WaitMove(on_check);
            }

            Ok(new_board)
        }
    }
}
