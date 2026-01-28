pub mod bias;
pub mod fifo;
pub mod soundcnt;

use crate::{
    apu::registers::{bias::Bias, fifo::Fifo, soundcnt::Soundcnt},
    bus::Bus,
};

#[derive(Debug, Default)]
pub struct ApuRegister {
    /// 0x080: Sound Bias (R/W)
    pub soundcnt: Soundcnt,
    /// 0x088: Sound Bias (R/W)
    pub bias: Bias,
    /// 0x0A0: Sound FIFO A (W)
    pub fifo_a: Fifo,
    /// 0x0A4: Sound FIFO A (W)
    pub fifo_b: Fifo,
}

impl Bus for ApuRegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            0x088..=0x089 => self.bias.value.read_byte(address),
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 0x0400_0000 {
            0x080..=0x083 => self.write_soundcnt(address, value),
            0x088..=0x089 => self.bias.value.write_byte(address, value),
            0x0A0..=0x0A3 => self.fifo_a.write_byte(address, value),
            0x0A4..=0x0A7 => self.fifo_b.write_byte(address, value),
            _ => {}
        }
    }
}

impl ApuRegister {
    fn write_soundcnt(&mut self, address: u32, value: u8) {
        self.soundcnt.write_byte(address, value);

        if self.soundcnt.reset_fifo_a() {
            self.fifo_a.buffer.clear();
        }

        if self.soundcnt.reset_fifo_b() {
            self.fifo_b.buffer.clear();
        }
    }
}
