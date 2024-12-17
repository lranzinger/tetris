use crate::{
    game::{GameState, HEIGHT, WIDTH},
    screen::ScreenConfig,
};
use macroquad::prelude::*;

const FONT_SIZE: f32 = 40.0;
const BUTTON_FONT_SIZE: f32 = 30.0;
const BUTTON_TEXT: &str = "Click to Restart";
const GAMEOVER_TEXT: &str = "Game Over!";
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
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            screen: ScreenConfig::new(),
        }
    }

    pub fn draw(&mut self, state: &GameState) {
        clear_background(BLACK);

        // Update screen config each frame for dynamic resizing
        self.screen = ScreenConfig::new();

        //Draw game
        self.draw_game_field();
        self.draw_placed_pieces(state);
        self.draw_current_piece(state);
        self.draw_scores(state.current_score, state.high_score);

        if state.game_over {
            self.draw_game_over();
        }
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

    fn get_restart_button_bounds(&self) -> ButtonBounds {
        let button_dims = measure_text(BUTTON_TEXT, None, BUTTON_FONT_SIZE as u16, 1.0);

        ButtonBounds {
            x: screen_width() / 2.0 - button_dims.width / 2.0 - 10.0,
            y: screen_height() / 2.0 + 30.0,
            width: button_dims.width + 20.0,
            height: button_dims.height + 20.0,
        }
    }

    fn draw_game_over(&mut self) {
        let text_dims = measure_text(GAMEOVER_TEXT, None, FONT_SIZE as u16, 1.0);
        draw_text(
            GAMEOVER_TEXT,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0,
            FONT_SIZE,
            WHITE,
        );

        let button = self.get_restart_button_bounds();

        draw_rectangle(button.x, button.y, button.width, button.height, DARKGRAY);
        draw_text(
            BUTTON_TEXT,
            button.x + 10.0,
            button.y + button.height - 10.0,
            BUTTON_FONT_SIZE,
            WHITE,
        );
    }

    pub fn check_restart_click(&self) -> bool {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return false;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let button = self.get_restart_button_bounds();

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

    fn draw_placed_pieces(&self, state: &GameState) {
        // Draw placed pieces
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = state.cells[y as usize][x as usize] {
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
}
