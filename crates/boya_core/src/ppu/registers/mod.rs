use crate::{
    bus::Bus,
    ppu::registers::{bgcnt::BgCnt, dispcnt::Dispcnt, dispstat::Dispstat},
};

pub mod bgcnt;
pub mod dispcnt;
pub mod dispstat;

#[derive(Debug, Default)]
pub struct PpuRegister {
    /// 0x000: LCD Control (R/W)
    pub dispcnt: Dispcnt,
    /// 0x004: General LCD Status (R/W)
    pub dispstat: Dispstat,
    /// 0x006: Vertical Counter (R)
    pub vcount: u16,
    /// 0x008: Background 0 Control (R/W)
    pub bg0cnt: BgCnt,
    /// 0x00A: Background 1 Control (R/W)
    pub bg1cnt: BgCnt,
    /// 0x00C: Background 2 Control (R/W)
    pub bg2cnt: BgCnt,
    /// 0x00E: Background 3 Control (R/W)
    pub bg3cnt: BgCnt,
    /// 0x010: Background 0 X-Offset (W)
    pub bg0hofs: u16,
    /// 0x012: Background 0 Y-Offset (W)
    pub bg0vofs: u16,
    /// 0x014: Background 1 X-Offset (W)
    pub bg1hofs: u16,
    /// 0x016: Background 1 Y-Offset (W)
    pub bg1vofs: u16,
    /// 0x018: Background 2 X-Offset (W)
    pub bg2hofs: u16,
    /// 0x01A: Background 2 Y-Offset (W)
    pub bg2vofs: u16,
    /// 0x01C: Background 3 X-Offset (W)
    pub bg3hofs: u16,
    /// 0x01E: Background 3 Y-Offset (W)
    pub bg3vofs: u16,
}

impl Bus for PpuRegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.read_byte(address),
            0x004 => self.dispstat.vcount,
            0x005 => self.dispstat.flags,
            0x006..=0x007 => self.vcount.read_byte(address),
            0x008..=0x009 => self.bg0cnt.value.read_byte(address),
            0x00A..=0x00B => self.bg1cnt.value.read_byte(address),
            0x00C..=0x00D => self.bg2cnt.value.read_byte(address),
            0x00E..=0x00F => self.bg3cnt.value.read_byte(address),
            _ => todo!("LCD registers read: {address:#08X}"),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.write_byte(address, value),
            0x004 => self.dispstat.vcount = value,
            0x005 => self.dispstat.write_flags(value),
            0x008..=0x009 => self.bg0cnt.value.write_byte(address, value),
            0x00A..=0x00B => self.bg1cnt.value.write_byte(address, value),
            0x00C..=0x00D => self.bg2cnt.value.write_byte(address, value),
            0x00E..=0x00F => self.bg3cnt.value.write_byte(address, value),
            0x010..=0x011 => self.bg0hofs.write_byte(address, value),
            0x012..=0x013 => self.bg0vofs.write_byte(address, value),
            0x014..=0x015 => self.bg1hofs.write_byte(address, value),
            0x016..=0x017 => self.bg1vofs.write_byte(address, value),
            0x018..=0x019 => self.bg2hofs.write_byte(address, value),
            0x01A..=0x01B => self.bg2vofs.write_byte(address, value),
            0x01C..=0x01D => self.bg3hofs.write_byte(address, value),
            0x01E..=0x01F => self.bg3vofs.write_byte(address, value),
            _ => todo!("LCD registers write: {address:#08X}"),
        }
    }
}
