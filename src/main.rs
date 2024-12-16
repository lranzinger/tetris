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
    current_score: u32,
    high_score: u32,
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
            current_score: 0,
            high_score: 0,
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
        let mut new_board = [[None; WIDTH as usize]; HEIGHT as usize];
        let mut new_row = HEIGHT as usize - 1;
        let mut lines_cleared = 0;
    
        // Scan from bottom up, skip full lines
        for y in (0..HEIGHT as usize).rev() {
            if !self.board[y].iter().all(|&cell| cell.is_some()) {
                new_board[new_row] = self.board[y];
                new_row = new_row.saturating_sub(1);
            } else {
                lines_cleared += 1;
            }
        }
    
        // Update score if lines were cleared
        if lines_cleared > 0 {
            let points = 100 * (1 << (lines_cleared - 1));
            self.current_score += points;
            self.high_score = self.high_score.max(self.current_score);
        }
    
        self.board = new_board;
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

    fn draw_block(&self, x: f32, y: f32, color: Color) {
        // Draw main block
        draw_rectangle(
            x * BLOCK_SIZE,
            y * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
            color
        );
        
        // Draw outline (1 pixel wide)
        let outline_color = Color::new(
            color.r * 0.7,
            color.g * 0.7,
            color.b * 0.7,
            1.0
        );
        
        draw_rectangle_lines(
            x * BLOCK_SIZE,
            y * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
            2.0,
            outline_color
        );
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        // Draw game field border and grid
        let field_width = WIDTH as f32 * BLOCK_SIZE;
        let field_height = HEIGHT as f32 * BLOCK_SIZE;
        
        // Draw main border
        draw_rectangle_lines(
            0.0,
            0.0,
            field_width,
            field_height,
            2.0,
            GRAY
        );

        // Draw grid lines
        for x in 0..WIDTH {
            draw_line(
                x as f32 * BLOCK_SIZE,
                0.0,
                x as f32 * BLOCK_SIZE,
                field_height,
                1.0,
                DARKGRAY
            );
        }
        
        for y in 0..HEIGHT {
            draw_line(
                0.0,
                y as f32 * BLOCK_SIZE,
                field_width,
                y as f32 * BLOCK_SIZE,
                1.0,
                DARKGRAY
            );
        }

        // Draw placed pieces with outline
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = self.board[y as usize][x as usize] {
                    self.draw_block(x as f32, y as f32, color);
                }
            }
        }

        // Draw current piece with outline
        for &(x, y) in &self.get_rotated_shape() {
            let draw_x = self.current_position.0 + x;
            let draw_y = self.current_position.1 + y;
            if draw_y >= 0 {
                self.draw_block(
                    draw_x as f32,
                    draw_y as f32,
                    self.current_piece.color()
                );
            }
        }

        if self.game_over {
            let text = "Game Over!";
            let button_text = "Click to Restart";
            let font_size = 30.0;
            let button_font_size = 20.0;

            // Draw game over text
            let text_dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0,
                font_size,
                WHITE,
            );

            // Draw restart button
            let button_dims = measure_text(button_text, None, button_font_size as u16, 1.0);
            let button_x = screen_width() / 2.0 - button_dims.width / 2.0 - 10.0;
            let button_y = screen_height() / 2.0 + 30.0;
            let button_width = button_dims.width + 20.0;
            let button_height = button_dims.height + 20.0;

            draw_rectangle(
                button_x,
                button_y,
                button_width,
                button_height,
                DARKGRAY,
            );
            draw_text(
                button_text,
                button_x + 10.0,
                button_y + button_height - 10.0,
                button_font_size,
                WHITE,
            );

            // Check for mouse click
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if mouse_pos.0 >= button_x 
                   && mouse_pos.0 <= button_x + button_width
                   && mouse_pos.1 >= button_y 
                   && mouse_pos.1 <= button_y + button_height {
                    self.restart();
                }
            }
        }

        // Draw scores
        let score_text = format!("Score: {}", self.current_score);
        let high_score_text = format!("High Score: {}", self.high_score);
        
        draw_text(
            &score_text,
            10.0,
            30.0,
            20.0,
            WHITE,
        );
        
        draw_text(
            &high_score_text,
            10.0,
            60.0,
            20.0,
            WHITE,
        );
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

    fn restart(&mut self) {
        self.board = [[None; WIDTH as usize]; HEIGHT as usize];
        self.current_score = 0;
        self.game_over = false;
        self.frame_count = 0;
        self.spawn_piece();
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