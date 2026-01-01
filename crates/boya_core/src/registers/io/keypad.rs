use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct KeyInput {
    pub value: u16,
}

#[derive(Debug, Default)]
pub struct KeyCnt {
    pub value: u16,
}

impl KeyCnt {
    pub fn irq_enable(&self) -> bool {
        self.value.has(14)
    }

    pub fn irq_condition(&self) -> KeyIrqCondition {
        match self.value.get(15) {
            0 => KeyIrqCondition::Or,
            _ => KeyIrqCondition::And,
        }
    }
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

#[derive(Debug)]
pub enum KeyIrqCondition {
    Or,
    And,
}
