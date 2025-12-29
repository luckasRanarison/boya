use crate::bus::{
    Bus,
    registers::{dma::Dma, keyinput::KeyInput, timer::Timer},
};

pub mod dma;
pub mod interrupt;
pub mod keyinput;
pub mod timer;

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
    /// 0x130: Key Status (R)
    pub keyinput: KeyInput,
    /// 0x200: Interrupt Enable (R/W)
    pub ie: u16,
    /// 0x202: Interrupt Request Flags (R/W)
    pub irf: u16,
    /// 0x208: Interrupt Master Enable (R/W)
    pub ime: u16,
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
            0x130..=0x131 => self.keyinput.value.read_byte(address),
            0x200..=0x201 => self.ie.read_byte(address),
            0x202..=0x203 => self.irf.read_byte(address),
            0x0400_0208..=0x0400_0209 => self.ime.read_byte(address),
            _ => 0, // open bus
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
            0x200..=0x201 => self.ie.write_byte(address, value),
            0x202..=0x203 => self.irf.write_byte(address, value),
            0x208..=0x209 => self.ime.write_byte(address, value),
            _ => {}
        }
    }
}

impl Bus for u16 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 2;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 2;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u16::from_le_bytes(bytes);
    }
}

impl Bus for u32 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 4;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 4;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u32::from_le_bytes(bytes);
    }
}
