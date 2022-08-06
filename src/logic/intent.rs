use crate::logic::board::BOARD_SIZE;
use crate::logic::basic::Coordinate;
use crate::utils::ValueError;

use std::str::Chars;

use super::game::GameState;
use super::pieces::PieceType;


pub struct PartialCoordinate {
    x: Option<usize>,
    y: Option<usize>,
}


impl PartialCoordinate {
    pub fn try_new(xo: Option<usize>, yo: Option<usize>) -> Result<Self, ValueError> {
        Ok(Self {
            x: xo.map(Coordinate::try_axis_bound).transpose()?,
            y: yo.map(Coordinate::try_axis_bound).transpose()?,
        })
    }

    pub fn xv(&self) -> Option<usize> { self.x }
    pub fn yv(&self) -> Option<usize> { self.y }

    pub fn to_complete(& self) -> Option<Coordinate> {
        match (self.x, self.y) {
            (Some(actual_x), Some(actual_y)) => Some(Coordinate::try_new(actual_x, actual_y).unwrap()),
            _ => None
        }
    }
}


pub enum Intent {
    Move(Option<PartialCoordinate>, Option<PartialCoordinate>),
    SelectPromotionType(PieceType),
    Surrender,
    Invalid,
    None,
}


impl Intent {
    pub fn from_partial_command(state: &GameState, cmd: &str) -> Self {
        match state {
            GameState::WaitMove => {
                match Self::try_parse_move(cmd) {
                    Ok(Some(intent)) => return intent,
                    Ok(None) => (),
                    Err(_) => (),
                };
        
                match Self::parse_surrender(cmd) {
                    Some(intent) => return intent,
                    None => (),
                };
            },
            GameState::SelectPromotionType(..) => {
                match Self::try_parse_select_promotion_type(cmd) {
                    Ok(Some(intent)) => return intent,
                    Ok(None) => (),
                    Err(_) => (),
                };
            },
        }

        match cmd.len() {
            0 => Self::None,
            _ => Self::Invalid,
        }
    }

    fn try_parse_select_promotion_type(cmd: &str) -> Result<Option<Self>, ValueError> {
        let mut chars = cmd.chars();

        Ok(match chars.next() {
            Some(c) => Some(Intent::SelectPromotionType(PieceType::from_letter(c)?)),
            None => None,
        })
    }

    fn try_parse_move(cmd: &str) -> Result<Option<Self>, ValueError> {
        let mut chars = cmd.chars();
        let first_coord = coordinate_from_chars(&mut chars)?;

        match first_coord {
            Some(first) => {
                let second_coord = coordinate_from_chars(&mut chars)?;

                match chars.next() {
                    None => Ok(Some(Self::Move(Some(first), second_coord))),
                    _ => Err(ValueError),
                }
        },
            None => Ok(None),
        }
    }

    fn parse_surrender(cmd: &str) -> Option<Self> {
        let mut chars = cmd.chars();

        match prefixes_from_chars(&mut chars, "surrender") {
            true => Some(Self::Surrender),
            false => None
        }
    }
}


fn coordinate_from_chars(chars: &mut Chars) -> Result<Option<PartialCoordinate>, ValueError> {
    let column = match chars.next() {
        Some(c) => Some(char_to_column(c)?),
        None => None,
    };

    let row = match chars.next() {
        Some(c) => Some(char_to_row(c)?),
        None => None,
    };

    match (column, row) {
        (None, None) => Ok(None),
        (c, r) => Some(PartialCoordinate::try_new(c, r)).transpose(),
    }
}


fn prefixes_from_chars(chars: &mut Chars, target_str: &str) -> bool {
    // But target on left side of zip to make sure
    // chars does not advance when target_str is done
    for (real, target) in target_str.chars().zip(chars) {
        if real != target {
            return false;
        }
    }
    true
}


fn char_to_column(letter: char) -> Result<usize, ValueError> {
    match letter {
        c @ 'A'..='H' => Ok(c as usize - 'A' as usize),
        _ => Err(ValueError),
    }
}


fn char_to_row(letter: char) -> Result<usize, ValueError> {
    match letter {
        c @ '1'..='8' => Ok(BOARD_SIZE - 1 - (c as usize - '1' as usize)),
        _ => Err(ValueError),
    }
}
