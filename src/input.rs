use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum InputState {
    None,
    MoveLeft,
    MoveRight,
    Rotate,
    Drop,
}

#[derive(Debug, Copy, Clone)]
pub struct TouchPosition {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time(f64);

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time(self.0 - other.0)
    }
}

pub struct InputHandler {
    touch_start: Option<(TouchPosition, Time)>,
    last_move_time: Time,
    key_hold_start: Option<(KeyCode, Time)>,
}

const HOLD_THRESHOLD: Time = Time(0.2);

impl InputHandler {
    pub fn new() -> Self {
        Self {
            touch_start: None,
            last_move_time: Time(0.0),
            key_hold_start: None,
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
        const MOVE_COOLDOWN: Time = Time(0.1);
        let current_time = Time(get_time());

        // Check for key press
        for key in [
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Down,
            KeyCode::Up,
            KeyCode::A,
            KeyCode::D,
            KeyCode::S,
            KeyCode::W,
        ] {
            if is_key_pressed(key) {
                self.key_hold_start = Some((key, current_time));
                match key {
                    KeyCode::Left | KeyCode::A => return InputState::MoveLeft,
                    KeyCode::Right | KeyCode::D => return InputState::MoveRight,
                    KeyCode::Up | KeyCode::W => return InputState::Rotate,
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
                        KeyCode::Left | KeyCode::A if elapsed > MOVE_COOLDOWN => {
                            self.last_move_time = current_time;
                            return InputState::MoveLeft;
                        }
                        KeyCode::Right | KeyCode::D if elapsed > MOVE_COOLDOWN => {
                            self.last_move_time = current_time;
                            return InputState::MoveRight;
                        }
                        KeyCode::Down | KeyCode::S => return InputState::Drop,
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
        const SWIPE_THRESHOLD: f32 = 30.0;

        let touches = touches();
        let current_time = Time(get_time());

        // Reset if no touches
        if touches.is_empty() {
            self.touch_start = None;
            return InputState::None;
        }

        let touch = &touches[0];
        match touch.phase {
            TouchPhase::Started => {
                self.touch_start = Some((
                    TouchPosition {
                        x: touch.position.x,
                        y: touch.position.y,
                    },
                    current_time,
                ));
            }
            TouchPhase::Moved => {
                if let Some((start_pos, _)) = self.touch_start {
                    let dx = touch.position.x - start_pos.x;
                    let dy = touch.position.y - start_pos.y;

                    if dx.abs() > SWIPE_THRESHOLD {
                        self.touch_start = None;
                        return if dx > 0.0 {
                            InputState::MoveRight
                        } else {
                            InputState::MoveLeft
                        };
                    }

                    if dy < -SWIPE_THRESHOLD {
                        self.touch_start = None;
                        return InputState::Rotate;
                    }
                }
            }
            TouchPhase::Stationary => {
                if let Some((_, start_time)) = self.touch_start {
                    if current_time - start_time > HOLD_THRESHOLD {
                        return InputState::Drop;
                    }
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.touch_start = None;
                return InputState::None;
            }
        }

        InputState::None
    }

    pub fn reset(&mut self) {
        self.touch_start = None;
        self.key_hold_start = None;
    }
}
