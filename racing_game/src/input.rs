use crate::window::*;

#[derive(PartialEq)]
pub enum InputKey{
    LEFT,
    RIGHT,
    UP
}

pub enum KeyState{
    PRESSED,
    RELEASED
}

pub struct Input{
    vertical : i32,
    horizontal : i32,

    button_states : Vec<(InputKey, bool)>
}

impl Input{
    pub fn new() -> Input {
        Input { vertical : 0, horizontal : 0, button_states : vec![(InputKey::LEFT, false), (InputKey::RIGHT, false), (InputKey::UP, false)] }
    }

    pub fn process(&mut self, window : &mut Window) {
        let events = window.update_events();

        for (key, state) in events {
            let state_bool = match state {
                KeyState::PRESSED => { true },
                KeyState::RELEASED => { false }
            };

            for button_state in &mut self.button_states {
                if button_state.0 == key { button_state.1 = state_bool; }
            }
        }
    }

    pub fn get_horizontal(&self) -> i32 {
        self.vertical
    }

    pub fn get_vertical(&self) -> i32 {
        if self.button_states[2].1 { 1 } else { 0 }
    }
}
