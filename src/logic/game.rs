use crate::logic::basic::{Coordinate};
use crate::logic::board::{Board, TileContent};

use super::pieces::{MoveError, Move, PieceType};


pub enum GameState {
    WaitMove,
    SelectPromotionType(Coordinate, Coordinate),
}


pub struct Game {
    pub board: Board,
    pub state: GameState,
}


impl Game {
    pub fn default() -> Self {
        Self {
            board: Board::default(),
            state: GameState::WaitMove,
        }
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
        let tile = self.board.get_tile(from);

        let moved = match tile {
            TileContent::Piece(piece) => piece.move_piece(
                &self.board,
                from,
                &Move::Promotion(to.clone(), piece_type.clone())
            ),
            TileContent::Empty => Err(MoveError::IllegalMove),
        };

        match moved {
            Ok(new_board) => {
                self.board = new_board;
                self.state = GameState::WaitMove;
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        let tile = self.board.get_tile(from);

        let moved = match tile {
            TileContent::Piece(piece) => piece.move_piece(
                &self.board,
                from,
                &Move::Regular(to.clone())
            ),
            TileContent::Empty => Err(MoveError::IllegalMove),
        };

        match moved {
            Ok(new_board) => {
                self.board = new_board;
                self.state = GameState::WaitMove;
                Ok(())
            },
            Err(MoveError::PromotionRequired) => {
                self.state = GameState::SelectPromotionType(from.clone(), to.clone());
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
}
