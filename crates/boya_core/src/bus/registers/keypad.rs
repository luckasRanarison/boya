use crate::{bus::types::Interrupt, utils::bitflags::Bitflag};

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

#[cfg(test)]
mod tests {
    use crate::{
        bus::{registers::keypad::Key, types::Interrupt},
        test::GbaTestBuilder,
        utils::bitflags::Bitflag,
    };

    #[test]
    fn test_keypad() {
        let asm = r"
            MOV     R0, #0x0400_0000
            MOV     R1, #0x130
            ADD     R1, #0x2
            MOV     R2, #0x0
            ORR     R2, #(1 shl 1)    ; set irq trigger B
            ORR     R2, #(1 shl 7)    ; set irq trigger Down
            ORR     R2, #(1 shl 14)   ; set irq enable
            ORR     R2, #(1 shl 15)   ; set irq condition to AND
            STRH    R2, [R0, R1]

            NOP
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.io.enable_master_irq();
                cpu.bus.io.enable_irq(Interrupt::Keypad);
                cpu.bus.io.keypad.keyinput.clear(Key::Down as u16);
                cpu.bus.io.keypad.keyinput.clear(Key::ButtonB as u16);
            })
            .assert_fn(|cpu| {
                assert!(cpu.bus.io.has_irq(Interrupt::Keypad), "keypad pending irq");
            })
            .run(10);
    }
}
