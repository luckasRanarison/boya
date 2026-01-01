use crate::{registers::io::interrupt::Interrupt, utils::bitflags::Bitflag};

#[derive(Debug)]
pub struct Keypad {
    pub keyinput: u16,
    pub keycnt: u16,
}

impl Default for Keypad {
    fn default() -> Self {
        Self {
            keyinput: 0x3FF,
            keycnt: 0,
        }
    }
}

impl Keypad {
    pub fn poll_interrupt(&self) -> Option<Interrupt> {
        if !self.irq_enable() {
            return None;
        }

        let keyinput = !(self.keyinput & 0x3FF);
        let keycnt = self.keycnt & 0x3FF;

        let result = match self.irq_condition() {
            KeyIrqCondition::Or => (keyinput | keycnt) != 0,
            KeyIrqCondition::And => (keyinput & keycnt) != 0,
        };

        result.then_some(Interrupt::Keypad)
    }

    pub fn irq_enable(&self) -> bool {
        self.keycnt.has(14)
    }

    pub fn irq_condition(&self) -> KeyIrqCondition {
        match self.keycnt.get(15) {
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
