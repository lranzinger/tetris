use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum InputState {
    None,
    MoveLeft,
    MoveRight,
    Rotate,
    Drop,
}

pub struct InputHandler {
    touch_start: Option<(f32, f32)>,
    touch_start_time: Option<f64>,
    touch_last_pos: Option<(f32, f32)>,
    last_swipe_time: f64,
    swipe_performed: bool,
    key_hold_start: Option<(KeyCode, f64)>,
    last_move_time: f64,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            touch_start: None,
            touch_start_time: None,
            touch_last_pos: None,
            last_swipe_time: 0.0,
            swipe_performed: false,
            key_hold_start: None,
            last_move_time: 0.0,
        }
    }

    pub fn update(&mut self) -> InputState {
        let touch_input = self.handle_touch();
        if touch_input != InputState::None {
            return touch_input;
        }

        let keyboard_input = self.handle_keyboard();
        if keyboard_input != InputState::None {
            return keyboard_input;
        }

        InputState::None
    }

    fn handle_keyboard(&mut self) -> InputState {
        const HOLD_THRESHOLD: f64 = 0.2;
        const MOVE_COOLDOWN: f64 = 0.1; // 100ms between moves
        let current_time = get_time();

        // Check for key press
        for key in [KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up] {
            if is_key_pressed(key) {
                self.key_hold_start = Some((key, current_time));
                match key {
                    KeyCode::Left => return InputState::MoveLeft,
                    KeyCode::Right => return InputState::MoveRight,
                    KeyCode::Up => return InputState::Rotate,
                    _ => (),
                }
            }
        }

        // Check for held keys
        if let Some((key, start_time)) = self.key_hold_start {
            if is_key_down(key) {
                if current_time - start_time > HOLD_THRESHOLD {
                    let elapsed = current_time - self.last_move_time;
                    match key {
                        KeyCode::Left if elapsed > MOVE_COOLDOWN => {
                            self.last_move_time = current_time;
                            return InputState::MoveLeft;
                        }
                        KeyCode::Right if elapsed > MOVE_COOLDOWN => {
                            self.last_move_time = current_time;
                            return InputState::MoveRight;
                        }
                        KeyCode::Down => return InputState::Drop,
                        _ => (),
                    }
                }
            } else {
                self.key_hold_start = None;
            }
        }

        InputState::None
    }

    fn handle_touch(&mut self) -> InputState {
        let current_time = get_time();
        const MOVE_THRESHOLD: f32 = 15.0;
        const SWIPE_THRESHOLD: f32 = 30.0;
        const HOLD_THRESHOLD: f64 = 0.1;
        const SWIPE_COOLDOWN: f64 = 0.2;

        // Handle lost focus or anomaly
        if !is_mouse_button_down(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Left) {
            self.reset_touch_state();
            return InputState::None;
        }

        // New touch/press - reset and initialize
        if is_mouse_button_pressed(MouseButton::Left) {
            self.reset_touch_state();
            self.touch_start = Some(mouse_position());
            self.touch_start_time = Some(current_time);
            self.touch_last_pos = Some(mouse_position());
            return InputState::None;
        }

        // Only process if we have valid start state
        if let (Some(start_pos), Some(start_time), Some(_)) =
            (self.touch_start, self.touch_start_time, self.touch_last_pos)
        {
            let (current_x, current_y) = mouse_position();
            let dx = current_x - start_pos.0;
            let dy = current_y - start_pos.1;

            // Prevent stuck states with timeout
            if current_time - start_time > 2.0 {
                // 2 second timeout
                self.reset_touch_state();
                return InputState::None;
            }

            // Handle swipe
            if !self.swipe_performed
                && current_time - self.last_swipe_time > SWIPE_COOLDOWN
                && (dx.abs() > SWIPE_THRESHOLD || dy.abs() > SWIPE_THRESHOLD)
            {
                self.swipe_performed = true;
                self.last_swipe_time = current_time;

                if dx.abs() > dy.abs() {
                    return if dx > 0.0 {
                        InputState::MoveRight
                    } else {
                        InputState::MoveLeft
                    };
                } else if dy < 0.0 {
                    return InputState::Rotate;
                }
            }

            // Handle hold
            if dx.abs() < MOVE_THRESHOLD
                && dy.abs() < MOVE_THRESHOLD
                && current_time - start_time > HOLD_THRESHOLD
            {
                return InputState::Drop;
            }
        } else {
            // Invalid state detected, reset
            self.reset_touch_state();
        }

        InputState::None
    }

    pub fn reset(&mut self) {
        self.key_hold_start = None;
        self.reset_touch_state();
    }

    fn reset_touch_state(&mut self) {
        self.touch_start = None;
        self.touch_start_time = None;
        self.touch_last_pos = None;
        self.swipe_performed = false;
    }
}
