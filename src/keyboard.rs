#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Space,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
}

impl Key {
    pub fn from_event_key(s: &str) -> Option<Key> {
        match s {
            "Space" => Some(Key::Space),
            "ArrowUp" => Some(Key::ArrowUp),
            "ArrowRight" => Some(Key::ArrowRight),
            "ArrowDown" => Some(Key::ArrowDown),
            "ArrowLeft" => Some(Key::ArrowLeft),
            // Pause, quit
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    pub fn from_event_type(event_type: String) -> Self {
        if event_type == "keydown" {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }
}
