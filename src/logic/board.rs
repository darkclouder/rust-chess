use std::mem;

use crate::logic::pieces::Piece;
use crate::logic::basic::{Coordinate, Player};
use crate::utils::DiscreetUnwrap;


pub const BOARD_SIZE: usize = 8;
pub const BOARD_MAX_AXIS: usize = BOARD_SIZE - 1;


#[derive(Clone)]
pub struct Board {
    pub tiles: [[TileContent; BOARD_SIZE]; BOARD_SIZE],
    pub turn: Player,
    pub en_passant: Option<Coordinate>,
}


impl Board {
    pub fn default() -> Self {
        Self::from_configuration(DEFAULT_PIECE_CONFIGURATION)
    }

    pub fn from_configuration(configuration: [[char; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        let tiles = configuration
            .into_iter()
            .map(|row| {
                row
                    .into_iter()
                    .map(|letter| {
                        TileContent::from_letter(letter)
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .duwrp()
            })
            .collect::<Vec<_>>()
            .try_into()
            .duwrp();

        Self {
            tiles,
            turn: Player::White,
            en_passant: None,
        }
    }

    pub fn turned(&self) -> Self {
        let mut new_board = self.clone();
        new_board.turn = match self.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
        new_board.en_passant = None;
        new_board
    }

    pub fn get_tile(&self, coordinate: &Coordinate) -> &TileContent {
        &self.tiles[coordinate.yv()][coordinate.xv()]
    }

    pub fn move_tile(&mut self, from: &Coordinate, to: &Coordinate) {
        let from_tile = mem::replace(
            &mut self.tiles[from.yv()][from.xv()],
            TileContent::Empty,
        );
        self.set_tile(to, from_tile);
    }

    pub fn clear_tile(&mut self, coordinate: &Coordinate) {
        self.set_tile(coordinate, TileContent::Empty);
    }

    pub fn set_tile(&mut self, coordinate: &Coordinate, new_tile: TileContent) {
        self.tiles[coordinate.yv()][coordinate.xv()] = new_tile;
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TileContent {
    Empty,
    Piece(Piece),
}


impl TileContent {
    pub fn from_letter(letter: char) -> Self {
        if letter == ' ' {
            return Self::Empty
        }

        Self::Piece(Piece::from_letter(letter).unwrap())
    }
}


const DEFAULT_PIECE_CONFIGURATION: [[char; BOARD_SIZE]; BOARD_SIZE] = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
];
