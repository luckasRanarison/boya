use crate::utils::register::Register;

#[derive(Debug, Default)]
pub struct SystemControl {
    pub ie: u16,      // 0x200  Interrupt Enable Register
    pub r#if: u16,    // 0x202  Interrupt Request Flags / IRQ Acknowledge
    pub waitcnt: u16, // 0x204  Game Pak Waitstate Control
    pub ime: u16,     // 0x208  Interrupt Master Enable Register
}

impl SystemControl {
    pub fn read(&self, address: usize) -> u8 {
        match address % 0x0400_0000 {
            0x200..=0x201 => self.ie.read(address),
            0x202..=0x203 => self.r#if.read(address),
            0x204..=0x205 => self.waitcnt.read(address),
            0x208..=0x209 => self.ime.read(address),
            _ => unreachable!("invalid system control read address: {address:#08X}"),
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address % 0x0400_0000 {
            0x200..=0x201 => self.ie.write(address, value),
            0x202..=0x203 => self.r#if.write(address, value),
            0x204..=0x205 => self.waitcnt.write(address, value),
            0x208..=0x209 => self.ime.write(address, value),
            _ => unreachable!("invalid system control write address: {address:#08X}"),
        }
    }
}
