use crate::{
    common::types::{MemoryAccess, Register},
    utils::bitflags::Bitflag,
};

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
            _ => unreachable!("invalid system control read address: {address:#010X}"),
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address % 0x0400_0000 {
            0x200..=0x201 => self.ie.write(address, value),
            0x202..=0x203 => self.r#if.write(address, value),
            0x204..=0x205 => self.waitcnt.write(address, value),
            0x208..=0x209 => self.ime.write(address, value),
            _ => unreachable!("invalid system control write address: {address:#010X}"),
        }
    }

    pub fn get_waitstate(&self, address: usize, access: MemoryAccess) -> u8 {
        let control_value = match address {
            0x0800_0000..=0x09FF_FFFF => self.waitcnt.get_bits(2, 4),
            0x0A00_0000..=0x0BFF_FFFF => self.waitcnt.get_bits(5, 7),
            0x0C00_0000..=0x0DFF_FFFF => self.waitcnt.get_bits(8, 10),
            0x0E00_0000..=0x0E00_FFFF => self.waitcnt.get_bits(0, 1),
            _ => unreachable!("invalid read waitstate address: {address:#010X}"),
        };

        match (address, control_value, access) {
            (_, 0b000, MemoryAccess::NonSeq) => 4,
            (_, 0b001, MemoryAccess::NonSeq) => 3,
            (_, 0b010, MemoryAccess::NonSeq) => 2,
            (_, 0b011, MemoryAccess::NonSeq) => 8,
            (0x0800_0000..=0x09FF_FFFF, 0b00..=0b11, MemoryAccess::Seq) => 2,
            (0x0A00_0000..=0x0BFF_FFFF, 0b00..=0b11, MemoryAccess::Seq) => 4,
            (0x0C00_0000..=0x0DFF_FFFF, 0b00..=0b11, MemoryAccess::Seq) => 8,
            (_, 0b00..=0b11, MemoryAccess::Seq) => 1,
            _ => unreachable!(),
        }
    }
}
