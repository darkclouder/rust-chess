pub mod draw;
pub mod logic;
pub mod utils;

use crate::draw::game_render::GameRenderer;
use crate::logic::game::Game;

fn main() {
    let mut game = Game::default();
    let mut renderer = GameRenderer::new(&mut game);
    renderer.run();
}
