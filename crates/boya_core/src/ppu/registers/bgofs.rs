use crate::bus::Bus;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bgofs {
    pub x: u16,
    pub y: u16,
}

impl Bus for Bgofs {
    fn read_byte(&self, _address: u32) -> u8 {
        0 // TODO: open bus
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 4 {
            0..1 => self.x.write_byte(address, value),
            _ => self.y.write_byte(address, value),
        }
    }
}
