use macroquad::prelude::*;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 20;
const BLOCK_SIZE: f32 = 30.0;

#[derive(Clone, Copy)]
enum Tetromino {
    I, O, T, S, Z, J, L,
}

impl Tetromino {
    fn shape(&self) -> Vec<(i32, i32)> {
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

    fn color(&self) -> Color {
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
}

struct Game {
    board: [[Option<Color>; WIDTH as usize]; HEIGHT as usize],
    current_piece: Tetromino,
    current_position: (i32, i32),
    frame_count: i32,
    rotation_state: i32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            board: [[None; WIDTH as usize]; HEIGHT as usize],
            current_piece: Tetromino::I,
            current_position: (WIDTH / 2 - 2, 0),
            frame_count: 0,
            rotation_state: 0,
            game_over: false,
        };
        game.spawn_piece();
        game
    }

    fn spawn_piece(&mut self) {
        let pieces = [
            Tetromino::I, Tetromino::O, Tetromino::T,
            Tetromino::S, Tetromino::Z, Tetromino::J, Tetromino::L,
        ];
        self.current_piece = pieces[rand::gen_range(0, pieces.len())];
        self.current_position = (WIDTH / 2 - 2, 0);
        self.rotation_state = 0;  // Reset rotation state
    }

    fn get_rotated_shape(&self) -> Vec<(i32, i32)> {
        let shape = self.current_piece.shape();
        
        // Calculate center of piece
        let center_x = shape.iter().map(|(x, _)| x).sum::<i32>() / shape.len() as i32;
        let center_y = shape.iter().map(|(_, y)| y).sum::<i32>() / shape.len() as i32;
        
        // Apply rotation around center
        match self.rotation_state {
            0 => shape,
            1 => shape.iter()
                     .map(|&(x, y)| {
                         let dx = x - center_x;
                         let dy = y - center_y;
                         (center_x - dy, center_y + dx)
                     })
                     .collect(),
            2 => shape.iter()
                     .map(|&(x, y)| {
                         let dx = x - center_x;
                         let dy = y - center_y;
                         (center_x - dx, center_y - dy)
                     })
                     .collect(),
            3 => shape.iter()
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
        for &(x, y) in &self.get_rotated_shape() {
            let new_x = self.current_position.0 + x + dx;
            let new_y = self.current_position.1 + y + dy;
            if new_x < 0 || new_x >= WIDTH || new_y >= HEIGHT || (new_y >= 0 && self.board[new_y as usize][new_x as usize].is_some()) {
                return false;
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for &(x, y) in &self.get_rotated_shape() {
            let board_x = self.current_position.0 + x;
            let board_y = self.current_position.1 + y;
            if board_y >= 0 {
                self.board[board_y as usize][board_x as usize] = Some(self.current_piece.color());
            }
        }
    }

    fn clear_lines(&mut self) {
        for y in (0..HEIGHT as usize).rev() {
            if self.board[y].iter().all(|&cell| cell.is_some()) {
                // Move all rows above down by one
                for row in (1..=y).rev() {
                    self.board[row] = self.board[row - 1];
                }
                // Clear the top row
                self.board[0] = [None; WIDTH as usize];
            }
        }
    }

    fn is_game_over(&self) -> bool {
        // Check if new piece overlaps with existing pieces
        for &(x, y) in &self.get_rotated_shape() {
            let board_x = self.current_position.0 + x;
            let board_y = self.current_position.1 + y;
            if board_y >= 0 && self.board[board_y as usize][board_x as usize].is_some() {
                return true;
            }
        }
        false
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.frame_count += 1;
        if self.frame_count % 20 == 0 {
            if self.can_move(0, 1) {
                self.current_position.1 += 1;
            } else {
                self.lock_piece();
                self.clear_lines();
                self.spawn_piece();
            }
        }

        if is_key_pressed(KeyCode::Left) && self.can_move(-1, 0) {
            self.current_position.0 -= 1;
        }
        if is_key_pressed(KeyCode::Right) && self.can_move(1, 0) {
            self.current_position.0 += 1;
        }
        if is_key_pressed(KeyCode::Down) && self.can_move(0, 1) {
            self.current_position.1 += 1;
        }
        if is_key_pressed(KeyCode::Up) {
            self.try_rotation();
        }

        if self.is_game_over() {
            self.game_over = true;
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = self.board[y as usize][x as usize] {
                    draw_rectangle(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                        color,
                    );
                }
            }
        }

        for &(x, y) in &self.get_rotated_shape() {
            let draw_x = self.current_position.0 + x;
            let draw_y = self.current_position.1 + y;
            if draw_y >= 0 {
                draw_rectangle(
                    draw_x as f32 * BLOCK_SIZE,
                    draw_y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    self.current_piece.color(),
                );
            }
        }

        if self.game_over {
            let text = "Game Over!";
            let font_size = 30.0;
            let text_dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0,
                font_size,
                WHITE,
            );
        }
    }

    fn try_rotation(&mut self) -> bool {
        let original_x = self.current_position.0;
        let offsets = [0, -1, 1, -2, 2]; // Check center, left, right positions

        let next_rotation = (self.rotation_state + 1) % 4;
        let temp_rotation = self.rotation_state;
        self.rotation_state = next_rotation;

        for &offset in &offsets {
            self.current_position.0 = original_x + offset;
            if self.is_valid_position() {
                return true;
            }
        }

        // If no valid position found, restore original position and rotation
        self.current_position.0 = original_x;
        self.rotation_state = temp_rotation;
        false
    }

    fn is_valid_position(&self) -> bool {
        for &(x, y) in &self.get_rotated_shape() {
            let new_x = self.current_position.0 + x;
            let new_y = self.current_position.1 + y;
            if new_x < 0 || new_x >= WIDTH || new_y >= HEIGHT || 
               (new_y >= 0 && self.board[new_y as usize][new_x as usize].is_some()) {
                return false;
            }
        }
        true
    }
}

#[macroquad::main("Tetris")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}