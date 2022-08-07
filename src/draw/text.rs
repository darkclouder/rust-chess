#[macro_export]
macro_rules! FORMAT_OUTPUT_TURN_SHORT {($a:expr) => { format!("It is {}'s turn.", $a) }}
#[macro_export]
macro_rules! FORMAT_OUTPUT_TURN {($a:expr) => { format!("It is {}'s turn.  Enter D2D3 to move from D2 to D3, surrender to give up, ^C to exit.", $a) }}

#[macro_export]
macro_rules! FORMAT_OUTPUT_ERROR_MOVE_FROM {($a:expr) => {
    format!("You cannot move from {}", $a)
}}
#[macro_export]
macro_rules! FORMAT_OUTPUT_ERROR_MOVE_FULL {($a:expr,$b:expr) => {
    format!("You cannot move from {} to {}", $a, $b)
}}


pub const OUTPUT_ENTER_MOVE: &str = "Press enter to move";
pub const OUTPUT_HINT_PROMOTE: &str = "Piece will be promoted. Select promotion type: (Q)ueen, (R)ook, K(N)ight or (B)ishop.";
pub const OUTPUT_ILLEGAL_MOVE: &str = "Illegal move";
pub const OUTPUT_MOVE_ERROR_CHECK: &str = "Cannot make this move because king is in check after this move.";
pub const OUTPUT_STATE_CHECK: &str = "Check!";

pub const LABEL_WHITE: &str = "White";
pub const LABEL_BLACK: &str = "Black";
