use crate::engine::window::Key;

pub trait KeyName {
    fn key_name(&self) -> Option<String>;
}

impl KeyName for Key {
    fn key_name(&self) -> Option<String> {
        let name = self.get_name();
        if name.is_some() { return name; }

        let name = match self {
            Key::Left => { "left" }
            Key::Right => { "right" }
            Key::Up => { "up" }
            Key::Down => { "down" }

            Key::LeftControl => { "lctrl" }
            Key::RightControl => { "rctrl" }

            Key::LeftShift => { "lshift" }
            Key::RightShift => { "rshift" }

            _ => { return None; }
        };
        
        Some(String::from(name))
    }
}
