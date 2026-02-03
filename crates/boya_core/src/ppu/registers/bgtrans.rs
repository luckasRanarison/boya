use crate::{bus::Bus, ppu::TransformParam};

#[derive(Debug, Default)]
pub struct Bgtrans {
    pub params: TransformParam,
}

impl Bus for Bgtrans {
    fn read_byte(&self, _address: u32) -> u8 {
        0 // TODO: open bus
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 16 {
            0..=1 => self.params.pa.write_byte(address, value),
            2..=3 => self.params.pb.write_byte(address, value),
            4..=5 => self.params.pc.write_byte(address, value),
            6..=7 => self.params.pd.write_byte(address, value),
            8..=11 => self.params.x.write_byte(address, value),
            _ => self.params.y.write_byte(address, value),
        }
    }
}
