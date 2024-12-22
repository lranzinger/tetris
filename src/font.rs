use macroquad::window::{screen_height, screen_width};

pub struct FontCache {
    pub size: f32,
    pub button_size: f32,
    pub debug_size: f32,
    pub stats_size: f32,
    last_screen_size: (f32, f32),
}

impl FontCache {
    pub fn new() -> Self {
        let mut font = Self {
            size: 0.0,
            button_size: 0.0,
            debug_size: 0.0,
            stats_size: 0.0,
            last_screen_size: (0.0, 0.0),
        };
        font.update();
        font
    }

    pub fn update(&mut self) {
        let current_size = (screen_width(), screen_height());
        if self.last_screen_size != current_size {
            self.size = current_size.1 * 0.06;
            self.button_size = self.size * 0.9;
            self.debug_size = self.size * 0.5;
            self.stats_size = self.size * 0.7;
            self.last_screen_size = current_size;
        }
    }
}
