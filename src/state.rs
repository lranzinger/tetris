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
    pub rotation: RotationState,
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

pub struct LevelState {
    pub current: usize,
    pub total_lines_cleared: u32,
}

pub struct GameState {
    pub status: GameStatus,
    pub score: ScoreState,
    pub dummy_board: Option<DummyBoard>,
    pub board: BoardState,
    pub piece: PieceState,
    pub timing: TimingState,
    pub level: LevelState,
}

impl GameState {
    pub fn new() -> Self {
        let initial_piece = Tetromino::random();
        Self {
            status: GameStatus::Start,
            score: ScoreState {
                current: 0,
                highest: 0,
            },
            dummy_board: Some(DummyBoard::new()),
            board: BoardState {
                cells: [[None; WIDTH as usize]; HEIGHT as usize],
                flashing_lines: Vec::new(),
            },
            piece: PieceState {
                typ: initial_piece,
                rotated: initial_piece.shape(),
                position: (WIDTH / 2 - 2, -1),
                rotation: RotationState::Zero,
            },
            timing: TimingState {
                fall_timer: 0.0,
                move_timer: 0.0,
                fall_interval: 0.48,
                move_interval: 0.1,
                line_clear_timer: 0.0,
            },
            level: LevelState {
                current: 0,
                total_lines_cleared: 0,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum RotationState {
    Zero = 0,
    Right = 1,
    Two = 2,
    Left = 3,
}

impl RotationState {
    pub fn next(&self) -> Self {
        match self {
            RotationState::Zero => RotationState::Right,
            RotationState::Right => RotationState::Two,
            RotationState::Two => RotationState::Left,
            RotationState::Left => RotationState::Zero,
        }
    }
}
