use macroquad::{
    text::{measure_text, TextDimensions},
    window::{screen_height, screen_width},
};

use crate::renderer::{LEVEL_TEXT, SCORE_TEXT};

pub struct FontCache {
    pub size: f32,
    pub button_size: f32,
    pub debug_size: f32,
    pub stats_size: f32,
}

impl FontCache {
    pub fn new() -> Self {
        let mut font = Self {
            size: 0.0,
            button_size: 0.0,
            debug_size: 0.0,
            stats_size: 0.0,
        };
        font.update();
        font
    }

    pub fn update(&mut self) {
        let current_size = (screen_width(), screen_height());
        self.size = current_size.1 * 0.06;
        self.button_size = self.size * 0.9;
        self.debug_size = self.size * 0.5;
        self.stats_size = self.size * 0.7;
    }
}

pub struct TextCache {
    pub score_label_dims: TextDimensions,
    pub level_label_dims: TextDimensions,
    number_widths: [TextDimensions; 10],
}

impl TextCache {
    pub fn new(font_size: u16) -> Self {
        let mut cache = Self {
            score_label_dims: TextDimensions::default(),
            level_label_dims: TextDimensions::default(),
            number_widths: [TextDimensions::default(); 10],
        };
        cache.update(font_size);
        cache
    }

    pub fn update(&mut self, font_size: u16) {
        self.score_label_dims = measure_text(SCORE_TEXT, None, font_size, 1.0);
        self.level_label_dims = measure_text(LEVEL_TEXT, None, font_size, 1.0);

        // Cache all single digit measurements
        for i in 0..10 {
            self.number_widths[i] = measure_text(&i.to_string(), None, font_size, 1.0);
        }
    }

    pub fn get_number_width(&self, num: u32) -> f32 {
        num.to_string()
            .chars()
            .map(|c| self.number_widths[c.to_digit(10).unwrap() as usize].width)
            .sum()
    }
}
