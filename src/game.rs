use crate::{
    input::{InputHandler, InputState},
    renderer::Renderer,
    tetromino::Tetromino,
};
use macroquad::prelude::*;

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 20;

pub struct GameState {
    pub cells: [[Option<Color>; WIDTH as usize]; HEIGHT as usize],
    pub current_score: u32,
    pub high_score: u32,
    pub current_piece: Tetromino,
    pub rotated_piece: Vec<(i32, i32)>,
    pub current_position: (i32, i32),
    pub frame_count: i32,
    pub rotation_state: i32,
    pub game_over: bool,
    pub fall_delay: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            cells: [[None; WIDTH as usize]; HEIGHT as usize],
            current_score: 0,
            high_score: 0,
            current_piece: Tetromino::random(),
            rotated_piece: vec![(0, 0)],
            current_position: (WIDTH / 2 - 2, 0),
            frame_count: 0,
            rotation_state: 0,
            game_over: false,
            fall_delay: 20,
        }
    }
}

pub struct Game {
    pub state: GameState,
    pub renderer: Renderer,
    pub input: InputHandler,
}
impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            renderer: Renderer::new(),
            input: InputHandler::new(),
        }
    }

    fn spawn_piece(&mut self) {
        self.state.current_piece = Tetromino::random();
        self.state.current_position = (WIDTH / 2 - 2, 0);
        self.state.rotation_state = 0; // Reset rotation state
    }

    fn get_rotated_shape(&self) -> Vec<(i32, i32)> {
        let shape = self.state.current_piece.shape();

        // Calculate center of piece
        let center_x = shape.iter().map(|(x, _)| x).sum::<i32>() / shape.len() as i32;
        let center_y = shape.iter().map(|(_, y)| y).sum::<i32>() / shape.len() as i32;

        // Apply rotation around center
        match self.state.rotation_state {
            0 => shape,
            1 => shape
                .iter()
                .map(|&(x, y)| {
                    let dx = x - center_x;
                    let dy = y - center_y;
                    (center_x - dy, center_y + dx)
                })
                .collect(),
            2 => shape
                .iter()
                .map(|&(x, y)| {
                    let dx = x - center_x;
                    let dy = y - center_y;
                    (center_x - dx, center_y - dy)
                })
                .collect(),
            3 => shape
                .iter()
                .map(|&(x, y)| {
                    let dx = x - center_x;
                    let dy = y - center_y;
                    (center_x + dy, center_y - dx)
                })
                .collect(),
            _ => shape,
        }
    }

    fn can_move(&self, dx: i32, dy: i32) -> bool {
        for &(x, y) in &self.state.rotated_piece {
            let new_x = self.state.current_position.0 + x + dx;
            let new_y = self.state.current_position.1 + y + dy;
            if !(0..WIDTH).contains(&new_x)
                || new_y >= HEIGHT
                || (new_y >= 0 && self.state.cells[new_y as usize][new_x as usize].is_some())
            {
                return false;
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for &(x, y) in &self.state.rotated_piece {
            let board_x = self.state.current_position.0 + x;
            let board_y = self.state.current_position.1 + y;
            if board_y >= 0 {
                self.state.cells[board_y as usize][board_x as usize] =
                    Some(self.state.current_piece.color());
            }
        }
        self.input.reset();
    }

    fn clear_lines(&mut self) {
        let mut new_board = [[None; WIDTH as usize]; HEIGHT as usize];
        let mut new_row = HEIGHT as usize - 1;
        let mut lines_cleared = 0;

        // Scan from bottom up, skip full lines
        for y in (0..HEIGHT as usize).rev() {
            if !self.state.cells[y].iter().all(|&cell| cell.is_some()) {
                new_board[new_row] = self.state.cells[y];
                new_row = new_row.saturating_sub(1);
            } else {
                lines_cleared += 1;
            }
        }

        // Update score if lines were cleared
        if lines_cleared > 0 {
            let points = 100 * (1 << (lines_cleared - 1));
            self.state.current_score += points;
            self.state.high_score = self.state.high_score.max(self.state.current_score);
        }

        self.state.cells = new_board;
    }

    fn is_game_over(&self) -> bool {
        // Check if new piece overlaps with existing pieces
        for &(x, y) in &self.state.rotated_piece {
            let board_x = self.state.current_position.0 + x;
            let board_y = self.state.current_position.1 + y;
            if board_y >= 0 && self.state.cells[board_y as usize][board_x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        if self.state.game_over {
            if self.renderer.check_restart_click() {
                self.restart();
            }
            return;
        }

        let input_state = self.input.update();
        self.handle_input(input_state);

        self.state.rotated_piece = self.get_rotated_shape();

        self.state.frame_count += 1;
        if self.state.frame_count % self.state.fall_delay == 0 {
            if self.can_move(0, 1) {
                self.state.current_position.1 += 1;
            } else {
                self.lock_piece();
                self.clear_lines();
                self.spawn_piece();
            }
        }

        if self.is_game_over() {
            self.state.game_over = true;
        }
    }

    fn handle_input(&mut self, input: InputState) {
        match input {
            InputState::MoveLeft => {
                if self.can_move(-1, 0) {
                    self.state.current_position.0 -= 1;
                }
            }
            InputState::MoveRight => {
                if self.can_move(1, 0) {
                    self.state.current_position.0 += 1;
                }
            }
            InputState::Rotate => {
                self.try_rotation();
            }
            InputState::Drop => {
                self.state.fall_delay = 2;
            }
            InputState::None => self.state.fall_delay = 20,
        }
    }
    fn try_rotation(&mut self) -> bool {
        let original_x = self.state.current_position.0;

        let offsets = [0, -1, 1, -2, 2];

        let next_rotation = (self.state.rotation_state + 1) % 4;
        let temp_rotation = self.state.rotation_state;
        self.state.rotation_state = next_rotation;
        self.state.rotated_piece = self.get_rotated_shape();

        for &offset in &offsets {
            self.state.current_position.0 = original_x + offset;
            if self.is_valid_position() {
                return true;
            }
        }

        // Restore original position and rotation if no valid position found
        self.state.current_position.0 = original_x;
        self.state.rotation_state = temp_rotation;
        self.state.rotated_piece = self.get_rotated_shape();
        false
    }

    fn is_valid_position(&self) -> bool {
        for &(x, y) in &self.state.rotated_piece {
            let new_x = self.state.current_position.0 + x;
            let new_y = self.state.current_position.1 + y;
            if new_x < 0
                || new_x >= WIDTH
                || new_y >= HEIGHT
                || (new_y >= 0 && self.state.cells[new_y as usize][new_x as usize].is_some())
            {
                return false;
            }
        }
        true
    }

    fn restart(&mut self) {
        self.state.cells = [[None; WIDTH as usize]; HEIGHT as usize];
        self.state.current_score = 0;
        self.state.game_over = false;
        self.state.frame_count = 0;
        self.spawn_piece();
    }
}
