use crate::{
    bus::{Bus, registers::dma::DmaTimer},
    utils::bitflags::Bitflag,
};

#[derive(Debug, Default)]
pub struct Soundcnt {
    pub cnt_l: u16,
    pub cnt_h: u16,
}

impl Soundcnt {
    pub fn timer_select_a(&self) -> DmaTimer {
        match self.cnt_h.get(10) {
            0 => DmaTimer::Timer0,
            _ => DmaTimer::Timer1,
        }
    }

    pub fn reset_fifo_a(&mut self) -> bool {
        self.cnt_h.take(11)
    }

    pub fn timer_select_b(&self) -> DmaTimer {
        match self.cnt_h.get(14) {
            0 => DmaTimer::Timer0,
            _ => DmaTimer::Timer1,
        }
    }

    pub fn reset_fifo_b(&mut self) -> bool {
        self.cnt_h.take(15)
    }
}

impl Bus for Soundcnt {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 4 {
            0..=1 => self.cnt_l.read_byte(address),
            _ => self.cnt_h.read_byte(address),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 4 {
            0..=1 => self.cnt_l.write_byte(address, value),
            _ => self.cnt_h.write_byte(address, value),
        }
    }
}
