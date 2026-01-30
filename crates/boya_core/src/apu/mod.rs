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

    pub fn on_timer_overflow(&mut self, timer: DmaTimer) {
        if self.registers.soundcnt.timer_select_a() == timer {
            if self.registers.fifo_a.needs_samples() {
                self.fifo_a_request = true;
            }

            self.registers.fifo_a.buffer.pop(); // TODO: implementation
        }

        if self.registers.soundcnt.timer_select_b() == timer {
            if self.registers.fifo_b.needs_samples() {
                self.fifo_b_request = true;
            }

            self.registers.fifo_b.buffer.pop(); // TODO: implementation
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
