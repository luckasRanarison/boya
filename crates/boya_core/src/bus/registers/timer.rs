use crate::{bus::Bus, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Timer {
    pub cnt_l: u16,
    pub cnt_h: u16,
}

impl Timer {
    pub fn prescaler_selection(&self) -> PrescalerSelection {
        match self.cnt_h.get_bits(0, 1) {
            0 => PrescalerSelection::F1,
            1 => PrescalerSelection::F64,
            2 => PrescalerSelection::F256,
            _ => PrescalerSelection::F1024,
        }
    }

    pub fn irq_enable(&self) -> bool {
        self.cnt_h.has(6)
    }

    pub fn is_operating(&self) -> bool {
        self.cnt_h.has(7)
    }
}

impl Bus for Timer {
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

#[derive(Debug)]
pub enum PrescalerSelection {
    F1,
    F64,
    F256,
    F1024,
}
