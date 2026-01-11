use crate::{
    bus::Bus,
    ppu::registers::{bgcnt::Bgcnt, bgofs::BgOfs, dispcnt::Dispcnt, dispstat::Dispstat},
};

pub mod bgcnt;
pub mod bgofs;
pub mod dispcnt;
pub mod dispstat;

#[derive(Debug, Default)]
pub struct PpuRegister {
    /// 0x000: LCD Control (R/W)
    pub dispcnt: Dispcnt,
    /// 0x002: Undocumented - Green Swap (R/W)
    pub greenswap: u16,
    /// 0x004: General LCD Status (R/W)
    pub dispstat: Dispstat,
    /// 0x006: Vertical Counter (R)
    pub vcount: u16,
    /// 0x008: Background 0-3 Control (R/W)
    pub bgcnt: [Bgcnt; 4],
    /// 0x010: Background 0 X-Offset (W)
    pub bgofs: [BgOfs; 4],
}

impl Bus for PpuRegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.read_byte(address),
            0x002..=0x003 => self.greenswap.read_byte(address),
            0x004 => self.dispstat.vcount,
            0x005 => self.dispstat.flags,
            0x006..=0x007 => self.vcount.read_byte(address),
            0x008..=0x009 => self.bgcnt[0].value.read_byte(address),
            0x00A..=0x00B => self.bgcnt[1].value.read_byte(address),
            0x00C..=0x00D => self.bgcnt[2].value.read_byte(address),
            0x00E..=0x00F => self.bgcnt[3].value.read_byte(address),
            _ => 0, // TODO: open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.write_byte(address, value),
            0x002..=0x003 => self.greenswap.write_byte(address, value),
            0x004 => self.dispstat.vcount = value,
            0x005 => self.dispstat.write_flags(value),
            0x008..=0x009 => self.bgcnt[0].value.write_byte(address, value),
            0x00A..=0x00B => self.bgcnt[1].value.write_byte(address, value),
            0x00C..=0x00D => self.bgcnt[2].value.write_byte(address, value),
            0x00E..=0x00F => self.bgcnt[3].value.write_byte(address, value),
            0x010..=0x013 => self.bgofs[0].write_byte(address, value),
            0x014..=0x017 => self.bgofs[1].write_byte(address, value),
            0x018..=0x01B => self.bgofs[2].write_byte(address, value),
            0x01C..=0x01F => self.bgofs[3].write_byte(address, value),
            _ => {}
        }
    }
}
