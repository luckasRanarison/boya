#[derive(Debug, Default)]
pub struct KeyInput {
    pub value: u16,
}

#[derive(Debug)]
pub enum Key {
    ButtonA,
    ButtonB,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
    ButtonR,
    ButtonL,
}
