pub mod logic;
pub mod draw;
pub mod utils;

use logic::board::Board;
use draw::board_render::BoardRenderer;
//use std::io::{stdout};
//use termion::raw::IntoRawMode;


fn main() {
    // Need to bind stdout to variable, otherwise raw mode is directly discarded again
    // let _stdout = stdout().into_raw_mode().unwrap();

    let board = Board::default();
    let mut renderer = BoardRenderer::new(&board);
    renderer.run();
}
