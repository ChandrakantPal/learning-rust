enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}
