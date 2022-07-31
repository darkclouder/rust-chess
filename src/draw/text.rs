#[macro_export]
macro_rules! FORMAT_OUTPUT_TURN {($a:expr) => { format!("It is {}'s turn.  Enter D2D3 to move from D2 to D3, surrender to give up, ^C to exit.", $a) }}
#[macro_export]
macro_rules! FORMAT_OUTPUT_ERROR_MOVE_A {($a:expr) => { format!("You cannot move from {}", $a) }}


pub const OUTPUT_ENTER_MOVE: &str = "Press enter to move";

pub const LABEL_WHITE: &str = "White";
pub const LABEL_BLACK: &str = "Black";
