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
