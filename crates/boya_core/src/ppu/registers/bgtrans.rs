use crate::bus::Bus;

#[derive(Debug)]
pub struct Bgtrans {
    pub pa: u16,
    pub pb: u16,
    pub pc: u16,
    pub pd: u16,
    pub x: u32,
    pub y: u32,
}

impl Default for Bgtrans {
    fn default() -> Self {
        Self {
            pa: 256,
            pb: 0,
            pc: 0,
            pd: 256,
            x: 0,
            y: 0,
        }
    }
}

impl Bus for Bgtrans {
    fn read_byte(&self, _address: u32) -> u8 {
        0 // TODO: open bus
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 16 {
            0..=1 => self.pa.write_byte(address, value),
            2..=3 => self.pb.write_byte(address, value),
            4..=5 => self.pc.write_byte(address, value),
            6..=7 => self.pd.write_byte(address, value),
            8..=11 => self.x.write_byte(address, value),
            _ => self.y.write_byte(address, value),
        }
    }
}
