mod game;
mod input;
mod renderer;
mod screen;
mod tetromino;

use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.renderer.draw(&game.state);
        next_frame().await;
    }
}
