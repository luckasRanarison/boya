use crate::{
    bus::{Bus, types::Interrupt},
    utils::bitflags::Bitflag,
};

#[derive(Debug, Default)]
pub struct Timer {
    pub cnt_l: u16,
    pub cnt_h: u16,

    unit: TimerUnit,
    counter: u16,
    divider: u32,
    overflow: bool,
    pending_irq: Option<Interrupt>,
}

impl Timer {
    pub fn new(unit: TimerUnit) -> Self {
        Self {
            unit,
            ..Default::default()
        }
    }

    pub fn irq_enable(&self) -> bool {
        self.cnt_h.has(6)
    }

    pub fn is_operating(&self) -> bool {
        self.cnt_h.has(7)
    }

    pub fn countup_timing(&self) -> bool {
        self.cnt_h.has(2)
    }

    pub fn tick(&mut self, cycles: u32, prev_overflow: bool) -> bool {
        if !self.is_operating() {
            return false;
        }

        if self.countup_timing() {
            if prev_overflow {
                self.increment();
            }
        } else {
            let step = self.clock_step();

            self.divider += cycles;

            while self.divider >= step {
                self.increment();
                self.divider -= step;
            }
        }

        self.poll_overflow()
    }

    pub fn poll_interrupt(&mut self) -> Option<Interrupt> {
        self.pending_irq.take()
    }

    fn increment(&mut self) {
        let (result, overflow) = self.counter.overflowing_add(1);

        if overflow {
            if self.irq_enable() {
                self.pending_irq = Some(self.unit.into());
            }

            self.counter = self.cnt_l;
            self.overflow = overflow;
        } else {
            self.counter = result;
        }
    }

    fn poll_overflow(&mut self) -> bool {
        std::mem::replace(&mut self.overflow, false)
    }

    fn clock_step(&self) -> u32 {
        match self.cnt_h.get_bits(0, 1) {
            0 => 1,
            1 => 64,
            2 => 256,
            _ => 1024,
        }
    }
}

impl Bus for Timer {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 4 {
            0..=1 => self.counter.read_byte(address),
            _ => self.cnt_h.read_byte(address),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 4 {
            0..=1 => self.cnt_l.write_byte(address, value),
            _ => {
                self.cnt_h.write_byte(address, value);

                if self.is_operating() {
                    self.counter = self.cnt_l;
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TimerUnit {
    #[default]
    Timer0,
    Timer1,
    Timer2,
    Timer3,
}

impl From<TimerUnit> for Interrupt {
    fn from(value: TimerUnit) -> Self {
        match value {
            TimerUnit::Timer0 => Interrupt::Timer0,
            TimerUnit::Timer1 => Interrupt::Timer1,
            TimerUnit::Timer2 => Interrupt::Timer2,
            TimerUnit::Timer3 => Interrupt::Timer3,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bus::types::Interrupt, test::AsmTestBuilder, utils::bitflags::Bitflag};

    #[test]
    fn test_timer() {
        let asm = r"
            _setup_timer1:
                MOV     R0, #0x0400_0000
                MOV     R1, #0x100
                ADD     R1, #0x6
                MOV     R2, #0x0
                ORR     R2, #(1 shl 2) ; set countup timing
                ORR     R2, #(1 shl 7) ; set start
                STRH    R2, [R0, R1]   ; 2 N (9)

            _setup_timer0:
                SUB     R1, #0x6
                MOV     R2, #0x10000
                SUB     R2, #0x100
                STRH    R2, [R0, R1]

                ADD     R1, #0x2
                MOV     R2, #0x0
                ORR     R2, #(1 shl 6) ; set enable irq
                ORR     R2, #(1 shl 7) ; set start
                STRH    R2, [R0, R1]   ; 2 N (9)
                NOP                    ; 1 S (6)

            loop:
                B       loop ; 2S + 1N (20)
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.io.ime = 1;
                cpu.bus.io.ie.set(Interrupt::Timer0 as u16);
            })
            .assert_fn(|cpu| {
                let timer0 = &cpu.bus.io.timer0;
                let timer1 = &cpu.bus.io.timer1;

                assert_eq!(0xFF00 + 19, timer0.counter, "timer 0 counter"); // + wrapped 20 cycles
                assert_eq!(0x1, timer1.counter, "timer 1 counter"); // + count-up cycle
                assert!(
                    cpu.bus.io.irf.has(Interrupt::Timer0 as u16),
                    "timer 0 pending irq"
                );
            })
            .run(17 + 13);
    }
}
