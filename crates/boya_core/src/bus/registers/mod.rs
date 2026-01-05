use crate::{
    bus::{
        Bus,
        registers::{
            dma::{Dma, DmaChannel},
            keypad::Keypad,
            timer::{Timer, TimerUnit},
            waitcnt::Waitcnt,
        },
        types::Interrupt,
    },
    utils::bitflags::Bitflag,
};

pub mod dma;
pub mod keypad;
pub mod timer;
pub mod waitcnt;

#[derive(Debug, Default)]
pub struct IORegister {
    /// 0x0B0: DMA 0 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma0: Dma,
    /// 0x0BC: DMA 1 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma1: Dma,
    /// 0x0C8: DMA 2 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma2: Dma,
    /// 0x0D4: DMA 3 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma3: Dma,
    /// 0x100: Timer 0 Control (R/W)
    pub timer0: Timer,
    /// 0x104: Timer 1 Control (R/W)
    pub timer1: Timer,
    /// 0x108: Timer 2 Control (R/W)
    pub timer2: Timer,
    /// 0x10C: Timer 3 Control (R/W)
    pub timer3: Timer,
    /// 0x130: Key Status (R), Key Interrupt Control (R/W)
    pub keypad: Keypad,
    /// 0x200: Interrupt Enable (R/W)
    pub ie: u16,
    /// 0x202: Interrupt Request Flags (R/W)
    pub irf: u16,
    /// 0x204: Waitstate Control (R/W)
    pub waitcnt: Waitcnt,
    /// 0x208: Interrupt Master Enable (R/W)
    pub ime: u16,
    /// 0x300: Power Down Control (R/W)
    pub haltcnt: u16,
    /// 0x800: Undocumented - Internal Memory Control (R/W)
    pub imemcnt: u32,
}

impl IORegister {
    pub fn new() -> Self {
        Self {
            dma1: Dma::new(DmaChannel::Dma1),
            dma2: Dma::new(DmaChannel::Dma2),
            dma3: Dma::new(DmaChannel::Dma3),
            timer1: Timer::new(TimerUnit::Timer1),
            timer2: Timer::new(TimerUnit::Timer2),
            timer3: Timer::new(TimerUnit::Timer3),
            ..Default::default()
        }
    }

    pub fn has_pending_irq(&self) -> bool {
        self.irf != 0
    }

    pub fn irq_master_enable(&self) -> bool {
        self.ime.has(0)
    }

    pub fn is_irq_enabled(&self, irq: Interrupt) -> bool {
        self.ie.has(irq as u16)
    }

    pub fn set_irq(&mut self, irq: Interrupt) {
        self.irf.set(irq as u16);
    }
}

impl Bus for IORegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            0x0BA..=0x0BB => self.dma0.read_byte(address),
            0x0C6..=0x0C7 => self.dma1.read_byte(address),
            0x0D2..=0x0D3 => self.dma2.read_byte(address),
            0x0DE..=0x0DF => self.dma3.read_byte(address),
            0x100..=0x103 => self.timer0.read_byte(address),
            0x104..=0x107 => self.timer1.read_byte(address),
            0x108..=0x10B => self.timer2.read_byte(address),
            0x10C..=0x10F => self.timer3.read_byte(address),
            0x130..=0x131 => self.keypad.keyinput.read_byte(address),
            0x132..=0x133 => self.keypad.keycnt.read_byte(address),
            0x200..=0x201 => self.ie.read_byte(address),
            0x202..=0x203 => self.irf.read_byte(address),
            0x204..=0x205 => self.waitcnt.value.read_byte(address),
            0x208..=0x209 => self.ime.read_byte(address),
            0x300..=0x301 => self.haltcnt.read_byte(address),
            0x800..=0x803 => self.imemcnt.read_byte(address),
            _ => 0, // TODO: open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 0x0400_0000 {
            0x0B0..=0x0BB => self.dma0.write_byte(address, value),
            0x0BC..=0x0C7 => self.dma1.write_byte(address, value),
            0x0C8..=0x0D3 => self.dma2.write_byte(address, value),
            0x0D4..=0x0DF => self.dma3.write_byte(address, value),
            0x100..=0x103 => self.timer0.write_byte(address, value),
            0x104..=0x107 => self.timer1.write_byte(address, value),
            0x108..=0x10B => self.timer2.write_byte(address, value),
            0x10C..=0x10F => self.timer3.write_byte(address, value),
            0x132..=0x133 => self.keypad.keycnt.write_byte(address, value),
            0x200..=0x201 => self.ie.write_byte(address, value),
            0x202..=0x203 => self.irf.write_byte(address, value),
            0x204..=0x205 => self.waitcnt.value.write_byte(address, value),
            0x208..=0x209 => self.ime.write_byte(address, value),
            0x300..=0x301 => self.haltcnt.write_byte(address, value),
            0x410..=0x411 => {} // undocumented, purpose unknown
            0x800..=0x803 => self.imemcnt.write_byte(address, value),
            _ => {}
        }
    }
}

#[cfg(test)]
impl IORegister {
    pub fn enable_master_irq(&mut self) {
        self.ime = 1;
    }

    pub fn enable_irq(&mut self, irq: Interrupt) {
        self.ie.set(irq as u16);
    }

    pub fn has_irq(&self, irq: Interrupt) -> bool {
        self.irf.has(irq as u16)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bus::types::Interrupt, test::GbaTestBuilder};

    #[test]
    fn test_irq_registers() {
        let asm = r"
            MOV     R0, #0x0400_0000
            MOV     R1, #0x200
            ADD     R1, #0x8
            MOV     R2, #0x1
            STRH    R2, [R0, R1]     ; set IME

            SUB     R1, #0x8
            MOV     R2, #0x0
            ORR     R2, #(1 shl 1)   ; set HBlank
            ORR     R2, #(1 shl 8)   ; set DMA 0
            STRH    R2, [R0, R1]     ; set IE
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_fn(|cpu| {
                assert!(cpu.bus.io.irq_master_enable(), "IME");
                assert!(cpu.bus.io.is_irq_enabled(Interrupt::HBlank), "HBlank IE");
                assert!(cpu.bus.io.is_irq_enabled(Interrupt::Dma0), "DMA0 IE");
            })
            .run(10);
    }
}
