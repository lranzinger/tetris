use crate::{
    cache::{FontCache, TextCache},
    game::{HEIGHT, WIDTH},
    screen::ScreenConfig,
    state::{Board, GameState, GameStatus, PieceState},
};
use macroquad::prelude::*;
use smallvec::{smallvec, SmallVec};

const START_TEXT: &str = "";
const START_BUTTON: &str = "Start";
const GAMEOVER_TEXT: &str = "Spiel vorbei";
const GAMEOVER_BUTTON: &str = "Neu starten";
pub const SCORE_TEXT: &str = "Score: ";
pub const LEVEL_TEXT: &str = "Level: ";
const HIGHSCORE_TEXT: &str = "Highscore: ";

struct ButtonBounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub struct Renderer {
    pub screen: ScreenConfig,
    text: TextCache,
    font: FontCache,
    last_fps_update: f64,
    current_fps: i32,
}

impl Renderer {
    pub fn new() -> Self {
        let font = FontCache::new();
        Self {
            screen: ScreenConfig::new(),
            text: TextCache::new(font.stats_size as u16),
            font,
            last_fps_update: 0.0,
            current_fps: 0,
        }
    }

    pub fn draw(&mut self, state: &GameState) {
        let current_size = (screen_width(), screen_height());
        if self.screen.size != current_size {
            self.screen = ScreenConfig::new();
            self.font.update();
            self.text.update(self.font.stats_size as u16);
        }

        self.draw_game_field();

        match state.status {
            GameStatus::Start => {
                if let Some(dummy_board) = &state.dummy_board {
                    self.draw_placed_pieces(&dummy_board.cells, &[]);
                }
                self.draw_start_screen();
            }
            GameStatus::Playing => {
                self.draw_placed_pieces(&state.board.cells, &state.board.flashing_lines);
                self.draw_current_piece(&state.piece);
                self.draw_stats(state.score.current, state.level.current);
            }
            GameStatus::GameOver => {
                self.draw_placed_pieces(&state.board.cells, &state.board.flashing_lines);
                self.draw_game_over(
                    state.score.current,
                    state.score.highest,
                    state.level.current,
                );
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
    fn get_button_bounds(&self, button_text: &str, font_size: f32) -> ButtonBounds {
        let button_dims = measure_text(button_text, None, font_size as u16, 1.0);

        let width = button_dims.width + screen_width() * 0.01;
        let height = button_dims.height + screen_height() * 0.02;

        ButtonBounds {
            x: screen_width() / 2.0 - width / 2.0,
            y: screen_height() / 2.0 + screen_height() * 0.02,
            width,
            height,
        }
    }

    fn draw_start_screen(&mut self) {
        let instructions = smallvec![
            "Links/Rechts: Bewegen",
            "Tippen: Drehen",
            "Halten: Fallen lassen",
        ];

        self.draw_overlay_screen(START_TEXT, START_BUTTON, instructions);
    }

    fn draw_game_over(&mut self, score: u32, high_score: u32, level: usize) {
        let score_text = [SCORE_TEXT, &score.to_string()].join("");
        let highscore_text = [HIGHSCORE_TEXT, &high_score.to_string()].join("");
        let level_text = [LEVEL_TEXT, &(level + 1).to_string()].join("");
        let scores = smallvec![
            score_text.as_str(),
            level_text.as_str(),
            highscore_text.as_str(),
        ];

        self.draw_overlay_screen(GAMEOVER_TEXT, GAMEOVER_BUTTON, scores);
    }

    fn draw_overlay_screen(
        &mut self,
        title: &str,
        button_text: &str,
        subtext: SmallVec<[&str; 3]>,
    ) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let center_x = screen_w / 2.0;
        let center_y = screen_h / 2.0;
        let spacing = screen_h * 0.05;

        // Background
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));

        // Title
        let title_dims = measure_text(title, None, self.font.size as u16, 1.0);
        draw_text(
            title,
            center_x - title_dims.width / 2.0,
            center_y - spacing,
            self.font.size,
            WHITE,
        );

        // Button
        let button = self.get_button_bounds(button_text, self.font.button_size);
        draw_rectangle(button.x, button.y, button.width, button.height, DARKGRAY);
        let button_dims = measure_text(button_text, None, self.font.button_size as u16, 1.0);
        draw_text(
            button_text,
            button.x + (button.width - button_dims.width) / 2.0,
            button.y + (button.height + button_dims.height) / 2.0,
            self.font.button_size,
            WHITE,
        );

        // Subtext
        let mut y = center_y + spacing * 3.0;
        for text in subtext {
            let dims = measure_text(text, None, self.font.stats_size as u16, 1.0);
            draw_text(
                text,
                center_x - dims.width / 2.0,
                y,
                self.font.stats_size,
                WHITE,
            );
            y += spacing;
        }
    }

    pub fn check_click(&self, status: GameStatus) -> bool {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return false;
        }

        let button_size = self.font.button_size;
        let button_text = match status {
            GameStatus::Start => START_BUTTON,
            GameStatus::GameOver => GAMEOVER_BUTTON,
            _ => return false,
        };
        let (mouse_x, mouse_y) = mouse_position();
        let button = self.get_button_bounds(button_text, button_size);

        mouse_x >= button.x
            && mouse_x <= button.x + button.width
            && mouse_y >= button.y
            && mouse_y <= button.y + button.height
    }

    fn draw_stats(&self, current_score: u32, level: usize) {
        let font_size = self.font.stats_size;
        let padding = 10.0;

        // Score drawing
        let score_num_width = self.text.get_number_width(current_score);
        let x_score = if self.screen.offset_x
            > self.text.score_label_dims.width + score_num_width + padding * 3.0
        {
            self.screen.offset_x
                - self.text.score_label_dims.width
                - score_num_width
                - padding * 2.0
        } else {
            padding
        };

        // Level drawing
        let level_num_width = self.text.get_number_width((level + 1) as u32);
        let total_level_width = self.text.level_label_dims.width + level_num_width;
        let game_field_right = self.screen.offset_x + (WIDTH as f32 * self.screen.block_size);
        let x_level = if screen_width() > game_field_right + total_level_width + padding * 3.0 {
            game_field_right + padding * 2.0
        } else {
            screen_width() - total_level_width - padding
        };

        let y = self.text.level_label_dims.height + padding;

        // Draw texts
        draw_text(SCORE_TEXT, x_score, y, font_size, WHITE);
        draw_text(
            &current_score.to_string(),
            x_score + self.text.score_label_dims.width,
            y,
            font_size,
            WHITE,
        );
        draw_text(LEVEL_TEXT, x_level, y, font_size, WHITE);
        draw_text(
            &(level + 1).to_string(),
            x_level + self.text.level_label_dims.width,
            y,
            font_size,
            WHITE,
        );
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

    fn draw_placed_pieces(&self, cells: &Board, flashing_lines: &[u8]) {
        let is_flash_frame = (get_time() * 10.0) as i32 % 2 == 0;
        for y in 0..HEIGHT as u8 {
            let is_line_flashing = flashing_lines.contains(&y);
            for x in 0..WIDTH as u8 {
                if let Some(color) = cells[y as usize][x as usize] {
                    let draw_color = if is_line_flashing && is_flash_frame {
                        WHITE
                    } else {
                        color
                    };
                    self.draw_block(x as f32, y as f32, draw_color);
                }
            }
        }
    }

    fn draw_current_piece(&self, piece: &PieceState) {
        for &(x, y) in &piece.rotated {
            let draw_x = piece.position.0 + x;
            let draw_y = piece.position.1 + y;
            if draw_y >= 0 {
                self.draw_block(draw_x as f32, draw_y as f32, piece.typ.color());
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

        let fps_text = self.current_fps.to_string();
        let padding: f32 = 10.0;

        let font_size = self.font.debug_size;
        let text_dims = measure_text(&fps_text, None, font_size as u16, 1.0);
        let x = screen_width() - text_dims.width - padding;
        let y = 2.5 * (text_dims.height + padding);

        draw_text(&fps_text, x, y, font_size, WHITE);
    }
}
