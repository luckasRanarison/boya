use crate::{bus::Bus, utils::collections::FifoBuffer};

#[derive(Debug, Default)]
pub struct Fifo {
    pub buffer: FifoBuffer<u8, 8>,
}

impl Fifo {
    pub fn needs_samples(&self) -> bool {
        self.buffer.len() > 0 && self.buffer.len() <= 4
    }
}

impl Bus for Fifo {
    fn read_byte(&self, _address: u32) -> u8 {
        0 // open-bus
    }

    fn write_byte(&mut self, _address: u32, value: u8) {
        self.buffer.push(value);
    }
}
