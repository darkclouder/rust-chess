use crate::logic::basic::{Coordinate};
use crate::logic::board::{Board, TileContent};


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
            TileContent::Empty => false,
            TileContent::Piece(piece) => {
                if piece.player == self.board.turn {
                    piece.can_move(&self.board, from, to)
                } else {
                    false
                }
            },
        }
    }
}
