use crate::{logic::basic::Player, utils::ValueError};

#[derive(Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}


impl PieceType {
    pub fn get_symbol(&self, player: &Player) -> &str {
        match player {
            Player::White => match self {
                Self::King => "\u{2654}",
                Self::Queen => "\u{2655}",
                Self::Rook => "\u{2656}",
                Self::Bishop => "\u{2657}",
                Self::Knight => "\u{2658}",
                Self::Pawn => "\u{2659}",
            },
            Player::Black => match self {
                Self::King => "\u{265A}",
                Self::Queen => "\u{265B}",
                Self::Rook => "\u{265C}",
                Self::Bishop => "\u{265D}",
                Self::Knight => "\u{265E}",
                Self::Pawn => "\u{265F}",
            },
        }
    }

    pub fn from_letter(letter: char) -> Result<Self, ValueError> {
        Ok(match letter {
            'K' => Self::King,
            'Q' => Self::Queen,
            'R' => Self::Rook,
            'B' => Self::Bishop,
            'N' => Self::Knight,
            'P' => Self::Pawn,
            _ => Err(ValueError)?
        })
    }
}


#[derive(Clone)]
pub struct Piece {
    pub player: Player,
    pub piece_type: PieceType,
}


impl Piece {
    pub fn from_letter(letter: char) -> Result<Self, ValueError> {
        let player = match letter {
            'A'..='Z' => Player::White,
            'a'..='z' => Player::Black,
            _ => Err(ValueError)?,
        };
        
        let upper_letter = letter.to_ascii_uppercase();
        let piece_type = PieceType::from_letter(upper_letter)?;

        Ok(Self {
            player,
            piece_type,
        })
    }

    pub fn get_symbol(&self) -> &str {
        self.piece_type.get_symbol(&self.player)
    }
}
