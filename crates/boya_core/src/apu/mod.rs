use crate::{apu::registers::ApuRegister, bus::registers::dma::DmaTimer, utils::Reset};

pub mod registers;

#[derive(Debug, Default)]
pub struct Apu {
    pub registers: ApuRegister,

    fifo_a_request: bool,
    fifo_b_request: bool,
}

impl Apu {
    pub fn step(&mut self) {}

    pub fn on_timer_overflow(&mut self, ovf0: bool, ovf1: bool) {
        if self.registers.fifo_a.needs_samples() {
            match self.registers.soundcnt.timer_select_a() {
                DmaTimer::Timer0 if ovf0 => self.fifo_a_request = true,
                DmaTimer::Timer1 if ovf1 => self.fifo_a_request = true,
                _ => {}
            }
        }

        if self.registers.fifo_b.needs_samples() {
            match self.registers.soundcnt.timer_select_b() {
                DmaTimer::Timer0 if ovf0 => self.fifo_b_request = true,
                DmaTimer::Timer1 if ovf1 => self.fifo_b_request = true,
                _ => {}
            }
        }
    }

    pub fn poll_fifo_a_request(&mut self) -> bool {
        std::mem::replace(&mut self.fifo_a_request, false)
    }

    pub fn poll_fifo_b_request(&mut self) -> bool {
        std::mem::replace(&mut self.fifo_b_request, false)
    }
}

impl Reset for Apu {
    fn reset(&mut self) {
        self.registers = ApuRegister::default();
        self.fifo_a_request = false;
        self.fifo_b_request = false;
    }
}
