use crate::{
    input::{InputHandler, InputState},
    renderer::Renderer,
    state::{GameState, GameStatus},
    tetromino::Tetromino,
};
use macroquad::prelude::*;

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 20;
const LINE_CLEAR_DURATION: f32 = 0.5;

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
        self.state.piece.typ = Tetromino::random();
        let shape = self.state.piece.typ.shape();
        let piece_width = shape.iter().map(|(x, _)| x).max().unwrap()
            - shape.iter().map(|(x, _)| x).min().unwrap()
            + 1;
        self.state.piece.position = (WIDTH / 2 - piece_width / 2, -1);
        self.state.piece.rotation = 0; // Reset rotation state
        self.state.piece.rotated = self.get_rotated_shape();
    }

    fn get_rotated_shape(&self) -> Vec<(i32, i32)> {
        let shape = self.state.piece.typ.shape();

        // Calculate center of piece
        let center_x = shape.iter().map(|(x, _)| x).sum::<i32>() / shape.len() as i32;
        let center_y = shape.iter().map(|(_, y)| y).sum::<i32>() / shape.len() as i32;

        // Apply rotation around center
        match self.state.piece.rotation {
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
        for &(x, y) in &self.state.piece.rotated {
            let new_x = self.state.piece.position.0 + x + dx;
            let new_y = self.state.piece.position.1 + y + dy;
            if !(0..WIDTH).contains(&new_x)
                || new_y >= HEIGHT
                || (new_y >= 0 && self.state.board.cells[new_y as usize][new_x as usize].is_some())
            {
                return false;
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for &(x, y) in &self.state.piece.rotated {
            let board_x = self.state.piece.position.0 + x;
            let board_y = self.state.piece.position.1 + y;
            if board_y >= 0 {
                self.state.board.cells[board_y as usize][board_x as usize] =
                    Some(self.state.piece.typ.color());
            }
        }
        self.input.reset();
    }

    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();

        // Identify full lines
        for y in 0..HEIGHT as usize {
            if self.state.board.cells[y].iter().all(|&cell| cell.is_some()) {
                lines_to_clear.push(y);
            }
        }

        if !lines_to_clear.is_empty() {
            // Start line clear animation
            self.state.board.flashing_lines = lines_to_clear;
            self.state.timing.line_clear_timer = LINE_CLEAR_DURATION;
        }
    }
    fn is_game_over(&self) -> bool {
        // Check if new piece overlaps with existing pieces
        for &(x, y) in &self.state.piece.rotated {
            let board_x = self.state.piece.position.0 + x;
            let board_y = self.state.piece.position.1 + y;
            if board_y >= 0 && self.state.board.cells[board_y as usize][board_x as usize].is_some()
            {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        match self.state.status {
            GameStatus::Start => {
                if self.renderer.check_click(GameStatus::Start) {
                    self.state.status = GameStatus::Playing;
                }
            }
            GameStatus::Playing => self.update_gameplay(),
            GameStatus::GameOver => {
                if self.renderer.check_click(GameStatus::GameOver) {
                    self.restart();
                    self.state.status = GameStatus::Playing;
                }
            }
        }
    }

    fn update_gameplay(&mut self) {
        let delta = get_frame_time();

        // Handle line clear animation
        if !self.state.board.flashing_lines.is_empty() {
            self.state.timing.line_clear_timer -= delta;
            if self.state.timing.line_clear_timer <= 0.0 {
                // Remove lines after flashing
                self.remove_flashing_lines();
                self.state.board.flashing_lines.clear();
            } else {
                // Skip further updates during line clear animation
                return;
            }
        }

        // Update timers
        self.state.timing.fall_timer += delta;
        self.state.timing.move_timer += delta;

        // Handle automatic piece falling
        if self.state.timing.fall_timer >= self.state.timing.fall_interval {
            self.state.timing.fall_timer = 0.0;
            if self.can_move(0, 1) {
                self.state.piece.position.1 += 1;
            } else {
                self.lock_piece();
                self.clear_lines();
                self.spawn_piece();
            }
        }

        // Handle input
        let input_state = self.input.update();
        self.handle_input(input_state);

        self.state.piece.rotated = self.get_rotated_shape();

        if self.is_game_over() {
            self.state.status = GameStatus::GameOver;
        }
    }

    fn handle_input(&mut self, input: InputState) {
        // Ignore input during lock-in or line clear animation
        if !self.state.board.flashing_lines.is_empty() {
            return;
        }

        match input {
            InputState::MoveLeft => {
                if self.state.timing.move_timer >= self.state.timing.move_interval {
                    if self.can_move(-1, 0) {
                        self.state.piece.position.0 -= 1;
                    }
                    self.state.timing.move_timer = 0.0;
                }
            }
            InputState::MoveRight => {
                if self.state.timing.move_timer >= self.state.timing.move_interval {
                    if self.can_move(1, 0) {
                        self.state.piece.position.0 += 1;
                    }
                    self.state.timing.move_timer = 0.0;
                }
            }
            InputState::Rotate => {
                self.try_rotation();
                self.state.timing.move_timer = 0.0;
            }
            InputState::Drop => {
                self.state.timing.fall_interval = 0.05; // Increase fall speed when dropping
            }
            InputState::None => {
                self.state.timing.fall_interval = 0.5; // Reset fall speed
            }
        }
    }

    fn try_rotation(&mut self) -> bool {
        if self.state.piece.typ == Tetromino::O {
            return false;
        }

        let original_x = self.state.piece.position.0;

        let offsets = [0, -1, 1, -2, 2];

        let next_rotation = (self.state.piece.rotation + 1) % 4;
        let temp_rotation = self.state.piece.rotation;
        self.state.piece.rotation = next_rotation;
        self.state.piece.rotated = self.get_rotated_shape();

        for &offset in &offsets {
            self.state.piece.position.0 = original_x + offset;
            if self.is_valid_position() {
                return true;
            }
        }

        // Restore original position and rotation if no valid position found
        self.state.piece.position.0 = original_x;
        self.state.piece.rotation = temp_rotation;
        self.state.piece.rotated = self.get_rotated_shape();
        false
    }

    fn is_valid_position(&self) -> bool {
        for &(x, y) in &self.state.piece.rotated {
            let new_x = self.state.piece.position.0 + x;
            let new_y = self.state.piece.position.1 + y;
            if new_x < 0
                || new_x >= WIDTH
                || new_y >= HEIGHT
                || (new_y >= 0 && self.state.board.cells[new_y as usize][new_x as usize].is_some())
            {
                return false;
            }
        }
        true
    }

    fn restart(&mut self) {
        let high_score = self.state.score.highest;
        let mut new_state = GameState::new();
        new_state.score.highest = high_score;
        self.state = new_state;
    }

    fn remove_flashing_lines(&mut self) {
        let mut new_board = [[None; WIDTH as usize]; HEIGHT as usize];
        let mut new_row = HEIGHT as usize - 1;

        // Copy the board, skipping the lines that were cleared
        for y in (0..HEIGHT as usize).rev() {
            if !self.state.board.flashing_lines.contains(&y) {
                new_board[new_row] = self.state.board.cells[y];
                new_row = new_row.saturating_sub(1);
            }
        }

        // Update score based on number of lines cleared
        let lines_cleared = self.state.board.flashing_lines.len();
        if lines_cleared > 0 {
            let points = 100 * (1 << (lines_cleared - 1));
            self.state.score.current += points;
            self.state.score.highest = self.state.score.current.max(self.state.score.current);
        }

        self.state.board.cells = new_board;
    }
}
