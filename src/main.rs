pub mod logic;
pub mod draw;
pub mod utils;

use crate::logic::game::Game;
use crate::draw::game_render::GameRenderer;


fn main() {
    let game = Game::default();
    let mut renderer = GameRenderer::new(&game);
    renderer.run();
}
