enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        InputEvent::Key(ev.keycode, ev.state)
    }
}
