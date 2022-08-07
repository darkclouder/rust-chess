use crate::logic::pieces::Piece;
use crate::logic::basic::{Coordinate, Player};
use crate::utils::DiscreetUnwrap;

use super::pieces::PieceType;


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
        let from_tile = match self.get_tile(&from) {
            TileContent::Empty => TileContent::Empty,
            TileContent::Piece(piece) => TileContent::Piece(piece.moved()),
        };
        self.clear_tile(from);
        self.set_tile(to, from_tile);
    }

    pub fn clear_tile(&mut self, coordinate: &Coordinate) {
        self.set_tile(coordinate, TileContent::Empty);
    }

    pub fn set_tile(&mut self, coordinate: &Coordinate, new_tile: TileContent) {
        self.tiles[coordinate.yv()][coordinate.xv()] = new_tile;
    }

    pub fn is_current_player_checkmate(&self) -> bool {
        let piece_coords = self.find_own_pieces(&self.turn);

        for coord in piece_coords {
            if let TileContent::Piece(piece) = self.get_tile(&coord) {
                for a_move in piece.all_moves(self, &coord) {
                    let new_board = piece.move_piece(self, &coord, &a_move).unwrap();
                    if !new_board.is_player_on_check(&self.turn) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_player_on_check(&self, player: &Player) -> bool {
        let (king_coord, enemy_coords) = self.find_pieces_for_check(player);

        // TODO: Refactor this soo we don't need to copy entire board for every check.
        let board = if *player == self.turn { self.turned() } else { self.clone() };

        for enemy_coord in enemy_coords {
            if let TileContent::Piece(piece) = self.get_tile(&enemy_coord) {
                if piece.can_move(&board, &enemy_coord, &king_coord) {
                    return true;
                }
            }
        }

        false
    }

    fn find_own_pieces(&self, player: &Player) -> Vec<Coordinate> {
        let mut coords: Vec<Coordinate> = Vec::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let TileContent::Piece(piece) = &self.tiles[y][x] {
                    if piece.player == *player {
                        coords.push(Coordinate::try_new(x, y).unwrap());
                    }
                }
            }
        }

        coords
    }

    fn find_pieces_for_check(&self, king_player: &Player) -> (Coordinate, Vec<Coordinate>) {
        let mut king_coord: Option<Coordinate> = None;
        let mut enemy_coords: Vec<Coordinate> = Vec::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let TileContent::Piece(piece) = &self.tiles[y][x] {
                    if piece.player == *king_player {
                        if matches!(piece.piece_type, PieceType::King) {
                            king_coord = Some(Coordinate::try_new(x, y).unwrap());
                        }
                    } else {
                        enemy_coords.push(Coordinate::try_new(x, y).unwrap())
                    }
                }
            }
        }

        (king_coord.unwrap(), enemy_coords)
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
    ['r', 'n', 'b', 'k', 'q', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
];
