use macroquad::prelude::*;

use crate::game::{HEIGHT, WIDTH};

pub struct ScreenConfig {
    pub block_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl ScreenConfig {
    pub fn new() -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Calculate optimal block size
        let scale_x = screen_width / WIDTH as f32;
        let scale_y = screen_height / HEIGHT as f32;
        let block_size: f32 = scale_x.min(scale_y) * 0.95; // 95% of available space

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
