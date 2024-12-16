use macroquad::prelude::*;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

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

struct ScreenConfig {
    block_size: f32,
    offset_x: f32,
    offset_y: f32,
}

impl ScreenConfig {
    fn new() -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Calculate optimal block size
        let scale_x = screen_width / WIDTH as f32;
        let scale_y = screen_height / HEIGHT as f32;
        let block_size = scale_x.min(scale_y) * 0.95; // 95% of available space
        
        // Center the game field
        let offset_x = (screen_width - (WIDTH as f32 * block_size)) / 2.0;
        let offset_y = (screen_height - (HEIGHT as f32 * block_size)) / 2.0;
        
        Self {
            block_size,
            offset_x,
            offset_y,
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
    touch_start: Option<(f32, f32)>,
    last_move_time: f64,
    keys_held: Vec<KeyCode>,
    screen: ScreenConfig,
    touch_start_time: Option<f64>,
    touch_last_pos: Option<(f32, f32)>,
    fall_delay: i32,
    touch_action_performed: bool,
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
            touch_start: None,
            last_move_time: 0.0,
            keys_held: Vec::new(),
            screen: ScreenConfig::new(),
            touch_start_time: None,
            touch_last_pos: None,
            fall_delay: 20,  // Default fall delay
            touch_action_performed: false,
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

    fn handle_touch(&mut self, current_time: f64) {
        let move_threshold = self.screen.block_size * 0.5;
        let swipe_threshold = self.screen.block_size * 1.0;

        if is_mouse_button_pressed(MouseButton::Left) {
            self.touch_start = Some(mouse_position());
            self.touch_start_time = Some(current_time);
            self.touch_last_pos = Some(mouse_position());
            self.touch_action_performed = false;
        } else if is_mouse_button_down(MouseButton::Left) {
            if let (Some(start_pos), Some(last_pos)) = (self.touch_start, self.touch_last_pos) {
                let (current_x, current_y) = mouse_position();
                let dx = current_x - start_pos.0;
                let dy = current_y - start_pos.1;
                
                let moved = (current_x - last_pos.0).abs() > move_threshold ||
                           (current_y - last_pos.1).abs() > move_threshold;

                if !moved && current_time - self.touch_start_time.unwrap() > 0.2 {
                    self.fall_delay = 5;
                } else if moved && !self.touch_action_performed {
                    self.fall_delay = 20;
                    if dx.abs() > dy.abs() {
                        if dx > swipe_threshold && self.can_move(1, 0) {
                            self.current_position.0 += 1;
                            self.touch_action_performed = true;
                        } else if dx < -swipe_threshold && self.can_move(-1, 0) {
                            self.current_position.0 -= 1;
                            self.touch_action_performed = true;
                        }
                    }
                    else if dy < -swipe_threshold {
                        self.try_rotation();
                        self.touch_action_performed = true;
                    }
                }
                self.touch_last_pos = Some((current_x, current_y));
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            self.touch_start = None;
            self.touch_start_time = None;
            self.touch_last_pos = None;
            self.fall_delay = 20;
            self.touch_action_performed = false;
        }
    }

    fn handle_input(&mut self) {
        let current_time = get_time();
        
        // Handle touch input
        self.handle_touch(current_time);

        // Handle keyboard input
            let move_delay = 0.2;
            
            // Handle keyboard input with holding
            for key in [KeyCode::Left, KeyCode::Right] {
                if is_key_pressed(key) {
                    self.keys_held.push(key);
                    self.last_move_time = current_time;
                    self.apply_move(key);
                }
            }

            // Handle held keys
            if current_time - self.last_move_time > move_delay {
                let keys_to_process: Vec<KeyCode> = self.keys_held.clone();
                for &key in &keys_to_process {
                    self.apply_move(key);
                }
                self.last_move_time = current_time;
            }
            if is_key_pressed(KeyCode::Down){
                self.fall_delay = 5;
            }

            // Remove released keys
            self.keys_held.retain(|&key| is_key_down(key));

            if is_key_released(KeyCode::Down){
                self.fall_delay = 20;
            }
            // Handle rotation (no holding)
            if is_key_pressed(KeyCode::Up) {
                self.try_rotation();
            }
        
    }

    fn apply_move(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => if self.can_move(-1, 0) {
                self.current_position.0 -= 1;
            },
            KeyCode::Right => if self.can_move(1, 0) {
                self.current_position.0 += 1;
            },
            _ => {}
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.handle_input();

        self.frame_count += 1;
        if self.frame_count % self.fall_delay == 0 {
            if self.can_move(0, 1) {
                self.current_position.1 += 1;
            } else {
                self.lock_piece();
                self.clear_lines();
                self.spawn_piece();
            }
        }

        if self.is_game_over() {
            self.game_over = true;
        }
    }

    fn draw_block(&self, x: f32, y: f32, color: Color) {
        let pos_x = self.screen.offset_x + x * self.screen.block_size;
        let pos_y = self.screen.offset_y + y * self.screen.block_size;
        let size = self.screen.block_size;
        
        // Draw main block with slight gradient
        let darker = Color::new(
            color.r * 0.8,
            color.g * 0.8,
            color.b * 0.8,
            1.0
        );
        
        // Draw block face
        draw_rectangle(
            pos_x,
            pos_y,
            size,
            size,
            color
        );
        
        // Draw inner shading for 3D effect
        draw_rectangle(
            pos_x + size * 0.1,
            pos_y + size * 0.1,
            size * 0.8,
            size * 0.8,
            darker
        );
        
        // Draw smooth outline
        draw_rectangle_lines(
            pos_x,
            pos_y,
            size,
            size,
            size * 0.1,  // Thicker lines
            Color::new(0.0, 0.0, 0.0, 0.5)  // Semi-transparent black
        );
        
        // Draw highlight
        draw_line(
            pos_x + size * 0.1,
            pos_y + size * 0.1,
            pos_x + size * 0.9,
            pos_y + size * 0.1,
            size * 0.05,
            Color::new(1.0, 1.0, 1.0, 0.3)
        );
    }

    fn draw(&mut self) {
        clear_background(BLACK);
        
        // Update screen config each frame for dynamic resizing
        self.screen = ScreenConfig::new();
        
        // Draw game field
        let field_width = WIDTH as f32 * self.screen.block_size;
        let field_height = HEIGHT as f32 * self.screen.block_size;
        
        // Draw border
        draw_rectangle_lines(
            self.screen.offset_x,
            self.screen.offset_y,
            field_width,
            field_height,
            2.0,
            GRAY
        );

        // Draw grid lines
        for x in 0..WIDTH {
            draw_line(
                self.screen.offset_x + x as f32 * self.screen.block_size,
                self.screen.offset_y,
                self.screen.offset_x + x as f32 * self.screen.block_size,
                self.screen.offset_y + field_height,
                1.0,
                DARKGRAY
            );
        }
        
        for y in 0..HEIGHT {
            draw_line(
                self.screen.offset_x,
                self.screen.offset_y + y as f32 * self.screen.block_size,
                self.screen.offset_x + field_width,
                self.screen.offset_y + y as f32 * self.screen.block_size,
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
