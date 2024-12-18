use macroquad::color::Color;

use crate::{
    dummy::DummyBoard,
    game::{HEIGHT, WIDTH},
    tetromino::Tetromino,
};

pub type Board = [[Option<Color>; WIDTH as usize]; HEIGHT as usize];

pub enum GameStatus {
    Start,
    Playing,
    GameOver,
}

pub struct PieceState {
    pub typ: Tetromino,
    pub position: (i32, i32),
    pub rotated: Vec<(i32, i32)>,
    pub rotation: i32,
}

pub struct TimingState {
    pub fall_interval: f32,
    pub fall_timer: f32,
    pub line_clear_timer: f32,
    pub move_interval: f32,
    pub move_timer: f32,
}

pub struct BoardState {
    pub cells: Board,
    pub flashing_lines: Vec<usize>,
}

pub struct ScoreState {
    pub current: u32,
    pub highest: u32,
}

pub struct GameState {
    pub board: BoardState,
    pub dummy_board: DummyBoard,
    pub piece: PieceState,
    pub score: ScoreState,
    pub status: GameStatus,
    pub timing: TimingState,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Start,
            score: ScoreState {
                current: 0,
                highest: 0,
            },
            dummy_board: DummyBoard::new(),
            board: BoardState {
                cells: [[None; WIDTH as usize]; HEIGHT as usize],

                flashing_lines: Vec::new(),
            },
            piece: PieceState {
                typ: Tetromino::random(),
                rotated: vec![(0, 0)],
                position: (WIDTH / 2 - 2, -1),
                rotation: 0,
            },
            timing: TimingState {
                fall_timer: 0.0,
                move_timer: 0.0,
                fall_interval: 0.5,
                move_interval: 0.1,
                line_clear_timer: 0.0,
            },
        }
    }
}
