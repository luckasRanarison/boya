use crate::bus::Bus;

#[derive(Debug, Default)]
pub struct ApuRegister {}

impl Bus for ApuRegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            _ => 0,
        }
    }

    fn write_byte(&mut self, address: u32, _value: u8) {
        match address % 0x0400_0000 {
            _ => {}
        }
    }
}
