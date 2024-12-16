use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum InputState {
    None,
    MoveLeft,
    MoveRight,
    Rotate,
    Drop,
}

pub fn update() -> InputState {
    let keyboard_input = handle_keyboard();
    if keyboard_input != InputState::None {
        return keyboard_input;
    }

    InputState::None
}

fn handle_keyboard() -> InputState {
    // Handle keyboard input with holding
    for key in [KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up] {
        if is_key_pressed(key) {
            match key {
                KeyCode::Left => return InputState::MoveLeft,
                KeyCode::Right => return InputState::MoveRight,
                KeyCode::Down => return InputState::Drop,
                KeyCode::Up => return InputState::Rotate,
                _ => (),
            }
        }
    }

    InputState::None
}

// fn handle_touch(&mut self, current_time: f64, state: &mut GameState) -> InputState {
//     const HOLD_DELAY: f64 = 0.3; // 300ms delay before fast fall
//     let move_threshold = self.screen.block_size * 0.2;
//     let swipe_threshold = self.screen.block_size * 0.5;

//     if is_mouse_button_pressed(MouseButton::Left) {
//         self.touch_start = Some(mouse_position());
//         self.touch_start_time = Some(current_time);
//         self.touch_last_pos = Some(mouse_position());
//         self.touch_action_performed = false;
//     } else if is_mouse_button_down(MouseButton::Left) {
//         if let (Some(start_pos), Some(last_pos)) = (self.touch_start, self.touch_last_pos) {
//             let (current_x, current_y) = mouse_position();
//             let dx = current_x - start_pos.0;
//             let dy = current_y - start_pos.1;

//             let moved = (current_x - last_pos.0).abs() > move_threshold
//                 || (current_y - last_pos.1).abs() > move_threshold;

//             if !moved && current_time - self.touch_start_time.unwrap() > HOLD_DELAY {
//                 return InputState::Drop;
//             } else if moved && !self.touch_action_performed {
//                 state.fall_delay = 20;
//                 if dx.abs() > dy.abs() {
//                     if dx > swipe_threshold {
//                         return InputState::MoveLeft;
//                     } else if dx < -swipe_threshold {
//                         return InputState::MoveRight;
//                     }
//                 } else if dy < -swipe_threshold {
//                     return InputState::Rotate;
//                 }
//             }
//             self.touch_last_pos = Some((current_x, current_y));
//         }
//     } else if is_mouse_button_released(MouseButton::Left) {
//         self.touch_start = None;
//         self.touch_start_time = None;
//         self.touch_last_pos = None;
//         self.touch_action_performed = false;
//     }
//     InputState::None
// }
