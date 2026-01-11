use crate::bus::Bus;

#[derive(Debug, Default, Clone, Copy)]
pub struct BgOfs {
    pub x: u16,
    pub y: u16,
}

impl Bus for BgOfs {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 4 {
            0..1 => self.x.read_byte(address),
            _ => self.y.read_byte(address),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 4 {
            0..1 => self.x.write_byte(address, value),
            _ => self.y.write_byte(address, value),
        }
    }
}
