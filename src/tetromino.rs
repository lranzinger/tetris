use macroquad::prelude::*;
use rand::gen_range;

#[derive(Clone, Copy, PartialEq)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetromino {
    pub fn shape(&self) -> Vec<(i32, i32)> {
        match self {
            Tetromino::I => vec![(0, 1), (1, 1), (2, 1), (3, 1)],
            Tetromino::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            Tetromino::T => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            Tetromino::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            Tetromino::Z => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            Tetromino::J => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            Tetromino::L => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Tetromino::I => BLUE,
            Tetromino::O => YELLOW,
            Tetromino::T => PURPLE,
            Tetromino::S => GREEN,
            Tetromino::Z => RED,
            Tetromino::J => ORANGE,
            Tetromino::L => PINK,
        }
    }

    pub fn random() -> Self {
        const PIECES: [Tetromino; 7] = [
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ];
        PIECES[gen_range(0, PIECES.len())]
    }
}
