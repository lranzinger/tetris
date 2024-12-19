use crate::{
    game::{HEIGHT, WIDTH},
    screen::ScreenConfig,
    state::{Board, GameState, GameStatus, LevelState, PieceState},
};
use macroquad::prelude::*;

const START_TEXT: &str = "";
const START_BUTTON: &str = "Start";
const GAMEOVER_TEXT: &str = "Spiel vorbei";
const GAMEOVER_BUTTON: &str = "Neu starten";
const SCORE_TEXT: &str = "Score:";
const HIGHSCORE_TEXT: &str = "Highscore:";

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
                if let Some(dummy_board) = &state.dummy_board {
                    self.draw_placed_pieces(&dummy_board.cells, &[]);
                }
                self.draw_start_screen();
            }
            GameStatus::Playing => {
                self.draw_placed_pieces(&state.board.cells, &state.board.flashing_lines);
                self.draw_current_piece(&state.piece);
                self.draw_score(state.score.current);
                self.draw_level_info(&state.level);
            }
            GameStatus::GameOver => {
                self.draw_placed_pieces(&state.board.cells, &state.board.flashing_lines);
                self.draw_game_over(state.score.current, state.score.highest);
                self.draw_level_info(&state.level);
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

    fn get_dynamic_font_size(&self) -> f32 {
        // Base font size on screen height
        let base_size = screen_height() * 0.05; // 5% of screen height
        base_size.max(20.0) // Minimum size of 20
    }

    fn get_button_font_size(&self) -> f32 {
        let base_size = self.get_dynamic_font_size() * 0.8; // Slightly smaller than main text
        base_size.max(16.0) // Minimum size of 16
    }

    fn draw_start_screen(&mut self) {
        let font_size = self.get_dynamic_font_size();
        let instruction_size = font_size * 0.8;
        let spacing = screen_height() * 0.05; // 5% of screen height for spacing

        self.draw_overlay_screen(START_TEXT, START_BUTTON);

        // Draw instructions
        let instructions = [
            "Links/Rechts: Bewegen",
            "Hoch: Drehen",
            "Halten: Fallen lassen",
        ];

        let mut y_offset = screen_height() / 2.0 + spacing * 2.0;
        for instruction in instructions {
            let text_dims = measure_text(instruction, None, instruction_size as u16, 1.0);
            draw_text(
                instruction,
                screen_width() / 2.0 - text_dims.width / 2.0,
                y_offset,
                instruction_size,
                WHITE,
            );
            y_offset += spacing;
        }
    }

    fn draw_game_over(&mut self, score: u32, high_score: u32) {
        self.draw_overlay_screen(GAMEOVER_TEXT, GAMEOVER_BUTTON);

        let font_size = self.get_dynamic_font_size() * 0.8;
        let spacing = screen_height() * 0.1; // 10% of screen height for spacing

        // Draw score
        let score_text = format!("{} {}", SCORE_TEXT, score);
        let text_dims = measure_text(&score_text, None, font_size as u16, 1.0);
        draw_text(
            &score_text,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0 + spacing,
            font_size,
            WHITE,
        );

        // Draw highscore
        let highscore_text = format!("{} {}", HIGHSCORE_TEXT, high_score);
        let text_dims = measure_text(&highscore_text, None, font_size as u16, 1.0);
        draw_text(
            &highscore_text,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0 + spacing * 1.5,
            font_size,
            WHITE,
        );
    }
    fn draw_overlay_screen(&mut self, title: &str, button_text: &str) {
        let font_size = self.get_dynamic_font_size();
        let button_size = self.get_button_font_size();

        // Draw overlay
        let overlay_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay_color);

        // Draw title
        let text_dims = measure_text(title, None, font_size as u16, 1.0);
        draw_text(
            title,
            screen_width() / 2.0 - text_dims.width / 2.0,
            screen_height() / 2.0,
            font_size,
            WHITE,
        );

        // Draw button
        let button = self.get_button_bounds(button_text, button_size);
        draw_rectangle(button.x, button.y, button.width, button.height, DARKGRAY);

        // Calculate text dimensions for centering
        let text_dims = measure_text(button_text, None, button_size as u16, 1.0);
        let text_x = button.x + (button.width - text_dims.width) / 2.0;
        let text_y = button.y + (button.height + text_dims.height) / 2.0;

        draw_text(button_text, text_x, text_y, button_size, WHITE);
    }

    pub fn check_click(&self, status: GameStatus) -> bool {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return false;
        }

        let button_size = self.get_button_font_size();
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

    fn draw_score(&self, current_score: u32) {
        let score_text = format!("{} {}", SCORE_TEXT, current_score);
        let font_size = self.get_dynamic_font_size();
        let padding: f32 = 10.0;

        let text_dims = measure_text(&score_text, None, font_size as u16, 1.0);
        let x = padding;
        let y = text_dims.height + padding;

        draw_text(&score_text, x, y, font_size, WHITE);
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

    fn draw_placed_pieces(&self, cells: &Board, flashing_lines: &[usize]) {
        for y in 0..HEIGHT {
            let y_usize = y as usize;
            let is_flashing = flashing_lines.contains(&y_usize);
            for x in 0..WIDTH {
                if let Some(color) = cells[y_usize][x as usize] {
                    let draw_color = if is_flashing && ((get_time() * 10.0) as i32 % 2 == 0) {
                        // Flashing effect: alternate between white and original color
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
    fn draw_level_info(&mut self, level: &LevelState) {
        let level_text = format!("Level: {}", level.current + 1);

        let padding: f32 = 10.0;
        let font_size = self.get_dynamic_font_size();
        let text_dims = measure_text(&level_text, None, font_size as u16, 1.0);
        let x = screen_width() - text_dims.width - padding;
        let y = text_dims.height + padding;

        draw_text(&level_text, x, y, font_size, WHITE);
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
        let padding: f32 = 10.0;

        let font_size = self.get_dynamic_font_size();
        let text_dims = measure_text(&fps_text, None, font_size as u16, 1.0);
        let x = screen_width() - text_dims.width - padding;
        let y = text_dims.height + padding;

        draw_text(&fps_text, x, y, font_size, WHITE);
    }
}
