use crate::logic::basic::{Coordinate};
use crate::logic::board::{Board, TileContent};

use super::pieces::MoveError;


pub struct Game {
    pub board: Board,
}


impl Game {
    pub fn default() -> Self {
        Self {
            board: Board::default(),
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
        let tile = self.board.get_tile(&from);
        
        match tile {
            TileContent::Piece(piece) => piece.can_move(&self.board, from, to),
            TileContent::Empty => false,
        }
    }

    pub fn move_piece(&mut self, from: &Coordinate, to: &Coordinate) -> Result<(), MoveError> {
        let tile = self.board.get_tile(&from);

        let new_board = match tile {
            TileContent::Piece(piece) => piece.move_piece(&self.board, from, to),
            TileContent::Empty => Err(MoveError),
        };

        self.board = new_board?;
        Ok(())
    }
}
