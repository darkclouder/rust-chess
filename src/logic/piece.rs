use crate::logic::basic::Player;


pub trait Piece {
    fn get_symbol(& self) -> &str;
    fn get_player(& self) -> &Player;
}


pub struct Pawn {
    pub player: Player,
}

impl Piece for Pawn {
    fn get_symbol(& self) -> &str { "P" }
    fn get_player(& self) -> &Player { &self.player }
}
