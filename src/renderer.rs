use crate::{
    game::{GameState, HEIGHT, WIDTH},
    screen::ScreenConfig,
};
use macroquad::prelude::*;

pub struct Renderer {
    pub screen: ScreenConfig,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            screen: ScreenConfig::new(),
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

    pub fn draw(&mut self, state: &GameState) {
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

        // Draw placed pieces with outline
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = state.cells[y as usize][x as usize] {
                    self.draw_block(x as f32, y as f32, color);
                }
            }
        }

        // Draw current piece with outline
        for &(x, y) in &state.rotated_piece {
            let draw_x = state.current_position.0 + x;
            let draw_y = state.current_position.1 + y;
            if draw_y >= 0 {
                self.draw_block(draw_x as f32, draw_y as f32, state.current_piece.color());
            }
        }
    }

    fn draw_game_over() {
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

        draw_rectangle(button_x, button_y, button_width, button_height, DARKGRAY);
        draw_text(
            button_text,
            button_x + 10.0,
            button_y + button_height - 10.0,
            button_font_size,
            WHITE,
        );

        // // Check for mouse click
        // if is_mouse_button_pressed(MouseButton::Left) {
        //     let mouse_pos = mouse_position();
        //     if mouse_pos.0 >= button_x
        //         && mouse_pos.0 <= button_x + button_width
        //         && mouse_pos.1 >= button_y
        //         && mouse_pos.1 <= button_y + button_height
        //     {
        //         self.restart();
        //     }
    }

    fn draw_scores(&self, current_score: u32, high_score: u32) {
        let score_text = format!("Score: {}", current_score);
        let high_score_text = format!("High Score: {}", high_score);

        draw_text(&score_text, 10.0, 30.0, 20.0, WHITE);
        draw_text(&high_score_text, 10.0, 60.0, 20.0, WHITE);
    }
}
