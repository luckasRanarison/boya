use crate::{
    bus::Bus,
    registers::io::{
        dma::{Dma, DmaChannel},
        interrupt::IrqRequestFlag,
        keypad::Keypad,
        timer::{Timer, TimerUnit},
        waitcnt::WaitCnt,
    },
};

pub mod dma;
pub mod interrupt;
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
    pub irf: IrqRequestFlag,
    /// 0x204: Waitstate Control (R/W)
    pub waitcnt: WaitCnt,
    /// 0x208: Interrupt Master Enable (R/W)
    pub ime: u16,
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
            0x202..=0x203 => self.irf.value.read_byte(address),
            0x204..=0x205 => self.waitcnt.value.read_byte(address),
            0x0400_0208..=0x0400_0209 => self.ime.read_byte(address),
            _ => todo!("I/O register read: {address:#08X}"),
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
            0x202..=0x203 => self.irf.value.write_byte(address, value),
            0x204..=0x205 => self.waitcnt.value.write_byte(address, value),
            0x208..=0x209 => self.ime.write_byte(address, value),
            _ => todo!("I/O register write: {address:#08X}"),
        }
    }
}
