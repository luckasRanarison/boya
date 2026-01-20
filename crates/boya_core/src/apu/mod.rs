use crate::{apu::registers::ApuRegister, utils::Reset};

pub mod registers;

#[derive(Debug)]
pub struct Apu {
    pub registers: ApuRegister,
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            registers: ApuRegister::default(),
        }
    }
}

impl Apu {
    pub fn step(&mut self) {}
}

impl Reset for Apu {
    fn reset(&mut self) {}
}
