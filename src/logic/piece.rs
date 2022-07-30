use crate::logic::basic::Player;


pub struct Piece {
    pub piece_type: Box<dyn PieceType>,
    pub player: Player,
}


pub trait PieceType {
    fn get_symbol(&self, piece: &Piece) -> &str;
}


pub struct King {}
impl King {
    pub fn new() -> Self { Self {} }
}
impl PieceType for King {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2654}",
            Player::Black => "\u{265A}",
        }
    }
}


pub struct Queen {}
impl Queen {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Queen {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2655}",
            Player::Black => "\u{265B}",
        }
    }
}


pub struct Rook {}
impl Rook {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Rook {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2656}",
            Player::Black => "\u{265C}",
        }
    }
}


pub struct Bishop {}
impl Bishop {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Bishop {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2657}",
            Player::Black => "\u{265D}",
        }
    }
}


pub struct Knight {}
impl Knight {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Knight {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2658}",
            Player::Black => "\u{265E}",
        }
    }
}


pub struct Pawn {}
impl Pawn {
    pub fn new() -> Self { Self {} }
}
impl PieceType for Pawn {
    fn get_symbol(&self, piece: &Piece) -> &str {
        match piece.player {
            Player::White => "\u{2659}",
            Player::Black => "\u{265F}",
        }
    }
}


impl Piece {
    pub fn get_symbol(& self) -> &str {
        self.piece_type.get_symbol(self)
    }
}
