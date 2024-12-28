use macroquad::prelude::*;

use crate::config::{Time, INPUT};

#[derive(PartialEq, Copy, Clone)]
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
}

pub struct InputHandler {
    touch_start: Option<(TouchPosition, Time)>,
    last_move_time: Time,
    key_hold_start: Option<(KeyCode, Time)>,
    is_moving: bool,
    is_dropping: bool,
    move_direction: Option<InputState>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            touch_start: None,
            last_move_time: Time(0.0),
            key_hold_start: None,
            is_moving: false,
            is_dropping: false,
            move_direction: None,
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
                if current_time - start_time > INPUT.hold_threshold {
                    let elapsed = current_time - self.last_move_time;
                    match key {
                        KeyCode::Left | KeyCode::A if elapsed > INPUT.move_cooldown => {
                            self.last_move_time = current_time;
                            return InputState::MoveLeft;
                        }
                        KeyCode::Right | KeyCode::D if elapsed > INPUT.move_cooldown => {
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
        let touches = touches();
        let current_time = Time(get_time());

        if touches.is_empty() {
            return InputState::None;
        }

        let touch = &touches[0];
        match touch.phase {
            TouchPhase::Started => {
                self.touch_start = Some((
                    TouchPosition {
                        x: touch.position.x,
                    },
                    current_time,
                ));
            }
            TouchPhase::Moved => {
                if self.is_dropping {
                    return InputState::Drop;
                }
                if let Some((start_pos, _)) = self.touch_start {
                    let dx = touch.position.x - start_pos.x;
                    if dx.abs() > INPUT.swipe_threshold {
                        let elapsed = current_time - self.last_move_time;
                        if elapsed > INPUT.move_cooldown_swipe {
                            self.last_move_time = current_time;
                            self.is_moving = true;
                            let direction = if dx > 0.0 {
                                InputState::MoveRight
                            } else {
                                InputState::MoveLeft
                            };
                            self.move_direction = Some(direction);
                            return direction;
                        }
                        return InputState::None;
                    }
                }
            }
            TouchPhase::Stationary => {
                if self.is_moving {
                    let elapsed = current_time - self.last_move_time;
                    if elapsed > INPUT.move_cooldown_hold {
                        self.last_move_time = current_time;
                        return self.move_direction.unwrap_or(InputState::None);
                    }
                } else if let Some((_, start_time)) = self.touch_start {
                    let touch_duration = current_time - start_time;
                    if touch_duration > INPUT.hold_threshold {
                        self.is_dropping = true;
                        return InputState::Drop;
                    }
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                if let Some((_, start_time)) = self.touch_start {
                    let touch_duration = current_time - start_time;
                    if touch_duration < INPUT.touch_threshold && !self.is_moving {
                        return InputState::Rotate;
                    }
                }
                self.touch_start = None;
                self.reset_movement();
                return InputState::None;
            }
        }
        InputState::None
    }

    fn reset_movement(&mut self) {
        self.is_moving = false;
        self.is_dropping = false;
        self.move_direction = None;
    }

    pub fn reset(&mut self) {
        self.touch_start = None;
        self.key_hold_start = None;
        self.reset_movement();
    }
}
