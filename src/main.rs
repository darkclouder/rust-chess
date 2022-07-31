pub mod logic;
pub mod draw;
pub mod utils;

use logic::board::Board;
use draw::game_render::GameRenderer;


fn main() {
    let board = Board::default();
    let mut renderer = GameRenderer::new(&board);
    renderer.run();
}
