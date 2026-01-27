pub mod bias;

use crate::{apu::registers::bias::Bias, bus::Bus};

#[derive(Debug, Default)]
pub struct ApuRegister {
    pub bias: Bias,
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
            0x088..=0x089 => self.bias.value.write_byte(address, value),
            _ => {}
        }
    }
}
