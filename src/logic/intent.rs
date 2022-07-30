use crate::utils::ValueError;

use std::str::Chars;


pub enum Intent {
    Move(Option<PartialCoordinate>, Option<PartialCoordinate>),
    Surrender,
    Invalid,
    None,
}


pub struct PartialCoordinate {
    pub x: Option<u16>,
    pub y: Option<u16>,
}


impl Intent {
    pub fn from_partial_command(cmd: &str) -> Self {
        match Self::try_parse_move(cmd) {
            Ok(Some(intent)) => return intent,
            Ok(None) => (),
            Err(_) => return Self::Invalid,
        };

        match Self::parse_surrender(cmd) {
            Some(intent) => return intent,
            None => (),
        };

        match cmd.len() {
            0 => Self::None,
            _ => Self::Invalid,
        }
    }

    fn try_parse_move(cmd: &str) -> Result<Option<Self>, ValueError> {
        let mut chars = cmd.chars();
        let first_coord = coordinate_from_chars(&mut chars)?;

        match first_coord {
            Some(first) => {
                if prefixes_from_chars(&mut chars, " to ") {
                    let second_coord = coordinate_from_chars(&mut chars)?;

                    match chars.next() {
                        None => Ok(Some(Self::Move(Some(first), second_coord))),
                        _ => Err(ValueError),
                    }
                } else {
                    Err(ValueError)
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
        (c, r) => Ok(Some(PartialCoordinate { x: c, y: r })),
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


fn char_to_column(letter: char) -> Result<u16, ValueError> {
    match letter {
        c @ 'A'..='H' => Ok(c as u16 - 'A' as u16),
        _ => Err(ValueError),
    }
}


fn char_to_row(letter: char) -> Result<u16, ValueError> {
    match letter {
        c @ '1'..='8' => Ok(8 - (c as u16 - '1' as u16)),
        _ => Err(ValueError),
    }
}
