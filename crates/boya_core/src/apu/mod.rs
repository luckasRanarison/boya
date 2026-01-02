use crate::apu::registers::ApuRegister;

pub mod registers;

#[derive(Debug)]
pub struct Apu {
    pub registers: ApuRegister,
}

// TODO: APU implementation
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
