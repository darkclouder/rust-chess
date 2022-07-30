pub mod logic;
pub mod draw;
pub mod utils;

use logic::board::Board;
use draw::board_render::BoardRenderer;


fn main() {
    let board = Board::default();
    let mut renderer = BoardRenderer::new(&board);
    renderer.run();
}
