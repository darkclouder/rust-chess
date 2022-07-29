use crate::logic::basic::Player;


pub enum PieceType {
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}


pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
}


impl Piece {
    pub fn get_symbol(& self) -> &str {
        match self.player {
            Player::White => match self.piece_type {
                PieceType::King => "\u{2654}",
                PieceType::Queen => "\u{2655}",
                PieceType::Rook => "\u{2656}",
                PieceType::Bishop => "\u{2657}",
                PieceType::Knight => "\u{2658}",
                PieceType::Pawn => "\u{2659}",
            },
            Player::Black => match self.piece_type {
                PieceType::King => "\u{265A}",
                PieceType::Queen => "\u{265B}",
                PieceType::Rook => "\u{265C}",
                PieceType::Bishop => "\u{265D}",
                PieceType::Knight => "\u{265E}",
                PieceType::Pawn => "\u{265F}",
            },
        }
    }
}
