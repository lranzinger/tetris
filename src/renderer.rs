use crate::{
    game::{GameState, GameStatus, HEIGHT, WIDTH},
    screen::ScreenConfig,
};
use macroquad::prelude::*;

const FONT_SIZE: f32 = 40.0;
const BUTTON_FONT_SIZE: f32 = 30.0;
const START_TEXT: &str = "Viel Spaß!";
const START_BUTTON: &str = "Start";
const GAMEOVER_TEXT: &str = "Spiel vorbei!";
const GAMEOVER_BUTTON: &str = "Neu starten";
const SCORE_TEXT: &str = "Score: ";
const HIGHSCORE_TEXT: &str = "Highscore: ";

struct ButtonBounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub struct Renderer {
    pub screen: ScreenConfig,
    last_fps_update: f64,
    current_fps: i32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            screen: ScreenConfig::new(),
            last_fps_update: 0.0,
            current_fps: 0,
        }
    }

    pub fn draw(&mut self, state: &GameState) {
        // Update screen config each frame for dynamic resizing
        self.screen = ScreenConfig::new();

        self.draw_game_field();

        match state.status {
            GameStatus::Start => {
                self.draw_placed_pieces(&state.dummy_board.cells);
                self.draw_start_screen();
            }
            GameStatus::Playing => {
                self.draw_placed_pieces(&state.cells);
                self.draw_current_piece(state);
                self.draw_scores(state.current_score, state.high_score);
            }
            GameStatus::GameOver => {
                self.draw_placed_pieces(&state.cells);
                self.draw_game_over(state.current_score, state.high_score);
            }
        }

        self.draw_debug_info();
    }

    fn draw_block(&self, x: f32, y: f32, color: Color) {
        let pos_x = self.screen.offset_x + x * self.screen.block_size;
        let pos_y = self.screen.offset_y + y * self.screen.block_size;
        let size = self.screen.block_size;

        // Draw main block with slight gradient
        let darker = Color::new(color.r * 0.8, color.g * 0.8, color.b * 0.8, 1.0);

        // Draw block face
        draw_rectangle(pos_x, pos_y, size, size, color);

        // Draw inner shading for 3D effect
        draw_rectangle(
            pos_x + size * 0.1,
            pos_y + size * 0.1,
            size * 0.8,
            size * 0.8,
            darker,
        );

        // Draw smooth outline
        draw_rectangle_lines(
            pos_x,
            pos_y,
            size,
            size,
            size * 0.1,                     // Thicker lines
            Color::new(0.0, 0.0, 0.0, 0.5), // Semi-transparent black
        );

        // Draw highlight
        draw_line(
            pos_x + size * 0.1,
            pos_y + size * 0.1,
            pos_x + size * 0.9,
            pos_y + size * 0.1,
            size * 0.05,
            Color::new(1.0, 1.0, 1.0, 0.3),
        );
    }

    fn get_button_bounds(&self, button_text: &str) -> ButtonBounds {
        let button_dims = measure_text(button_text, None, BUTTON_FONT_SIZE as u16, 1.0);

        ButtonBounds {
            x: screen_width() / 2.0 - button_dims.width / 2.0 - 10.0,
            y: screen_height() / 2.0 + 30.0,
            width: button_dims.width + 20.0,
            height: button_dims.height + 20.0,
        }
    }

    fn draw_start_screen(&mut self) {
        self.draw_overlay_screen(START_TEXT, START_BUTTON);
    }

    fn draw_game_over(&mut self, score: u32, high_score: u32) {
        self.draw_overlay_screen(GAMEOVER_TEXT, GAMEOVER_BUTTON);

        let score_text = format!("{} {}", SCORE_TEXT, score);
        let text_dims = measure_text(&score_text, None, FONT_SIZE as u16, 1.0);
        draw_text(
            &score_text,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0 + 130.0,
            FONT_SIZE - 5.0,
            WHITE,
        );

        let highscore_text = format!("{} {}", HIGHSCORE_TEXT, high_score);
        let text_dims = measure_text(&highscore_text, None, FONT_SIZE as u16, 1.0);
        draw_text(
            &highscore_text,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0 + 170.0,
            FONT_SIZE - 5.0,
            WHITE,
        );
    }
    fn draw_overlay_screen(&mut self, title: &str, button_text: &str) {
        let overlay_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);

        let text_dims = measure_text(title, None, FONT_SIZE as u16, 1.0);
        draw_text(
            title,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0,
            FONT_SIZE,
            WHITE,
        );

        let button = self.get_button_bounds(button_text);

        draw_rectangle(button.x, button.y, button.width, button.height, DARKGRAY);
        draw_text(
            button_text,
            button.x + 10.0,
            button.y + button.height - 10.0,
            BUTTON_FONT_SIZE,
            WHITE,
        );
    }

    pub fn check_click(&self, status: GameStatus) -> bool {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return false;
        }

        let button_text = match status {
            GameStatus::Start => START_BUTTON,
            GameStatus::GameOver => GAMEOVER_BUTTON,
            _ => return false,
        };
        let (mouse_x, mouse_y) = mouse_position();
        let button = self.get_button_bounds(button_text);

        mouse_x >= button.x
            && mouse_x <= button.x + button.width
            && mouse_y >= button.y
            && mouse_y <= button.y + button.height
    }

    fn draw_scores(&self, current_score: u32, high_score: u32) {
        let score_text = format!("{} {}", SCORE_TEXT, current_score);
        let high_score_text = format!("{} {}", HIGHSCORE_TEXT, high_score);

        draw_text(&score_text, 10.0, 30.0, FONT_SIZE, WHITE);
        draw_text(&high_score_text, 10.0, 60.0, FONT_SIZE, WHITE);
    }

    fn draw_game_field(&self) {
        let field_width = WIDTH as f32 * self.screen.block_size;
        let field_height = HEIGHT as f32 * self.screen.block_size;

        // Draw border
        draw_rectangle_lines(
            self.screen.offset_x,
            self.screen.offset_y,
            field_width,
            field_height,
            2.0,
            GRAY,
        );

        // Draw grid lines
        for x in 0..WIDTH {
            draw_line(
                self.screen.offset_x + x as f32 * self.screen.block_size,
                self.screen.offset_y,
                self.screen.offset_x + x as f32 * self.screen.block_size,
                self.screen.offset_y + field_height,
                1.0,
                DARKGRAY,
            );
        }

        for y in 0..HEIGHT {
            draw_line(
                self.screen.offset_x,
                self.screen.offset_y + y as f32 * self.screen.block_size,
                self.screen.offset_x + field_width,
                self.screen.offset_y + y as f32 * self.screen.block_size,
                1.0,
                DARKGRAY,
            );
        }
    }

    fn draw_placed_pieces(&self, cells: &[[Option<Color>; WIDTH as usize]; HEIGHT as usize]) {
        // Draw placed pieces
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = cells[y as usize][x as usize] {
                    self.draw_block(x as f32, y as f32, color);
                }
            }
        }
    }

    fn draw_current_piece(&self, state: &GameState) {
        for &(x, y) in &state.rotated_piece {
            let draw_x = state.current_position.0 + x;
            let draw_y = state.current_position.1 + y;
            if draw_y >= 0 {
                self.draw_block(draw_x as f32, draw_y as f32, state.current_piece.color());
            }
        }
    }

    fn draw_debug_info(&mut self) {
        if !cfg!(debug_assertions) {
            return;
        }

        let current_time = get_time();

        // Update FPS once per second
        if current_time - self.last_fps_update >= 1.0 {
            self.current_fps = get_fps();
            self.last_fps_update = current_time;
        }

        let fps_text = format!("FPS: {}", self.current_fps);
        let padding = 10.0;

        let text_dims = measure_text(&fps_text, None, FONT_SIZE as u16, 1.0);
        let x = screen_width() - text_dims.width - padding;
        let y = text_dims.height + padding;

        draw_text(&fps_text, x, y, FONT_SIZE, WHITE);
    }
}
