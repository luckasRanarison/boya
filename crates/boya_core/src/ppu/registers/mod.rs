use crate::{
    bus::Bus,
    ppu::registers::{
        bgcnt::Bgcnt,
        bgofs::Bgofs,
        bgtrans::Bgtrans,
        bldalpha::Bldalpha,
        bldcnt::Bldcnt,
        bldy::Bldy,
        dispcnt::Dispcnt,
        dispstat::Dispstat,
        mosaic::Mosaic,
        window::{WinH, WinV, Winin, Winout},
    },
};

pub mod bgcnt;
pub mod bgofs;
pub mod bgtrans;
pub mod bldalpha;
pub mod bldcnt;
pub mod bldy;
pub mod dispcnt;
pub mod dispstat;
pub mod mosaic;
pub mod window;

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
    pub bgofs: [Bgofs; 4],
    /// 0x020: Background 2 Transform parameters (W)
    pub bg2trans: Bgtrans,
    /// 0x030: Background 2 Transform parameters (W)
    pub bg3trans: Bgtrans,
    /// 0x040: Window 0 Horizontal Dimension (W)
    pub winh: [WinH; 2],
    /// 0x044: Window 0 Vertical Dimension (W)
    pub winv: [WinV; 2],
    /// 0x048: Control of Inside of Window (R/W)
    pub winin: Winin,
    /// 0x04A: Control of Outiside of Window (R/W)
    pub winout: Winout,
    /// 0x04C: Mosaic Function (W)
    pub mosaic: Mosaic,
    /// 0x050: Color Special Effects Selection (R/W)
    pub bldcnt: Bldcnt,
    /// 0x052: Alpha Blending Coefficients (W)
    pub bldalpha: Bldalpha,
    /// 0x054: Brightness (Fade-In/Out) Coefficients (W)
    pub bldy: Bldy,
}

impl Bus for PpuRegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.read_byte(address),
            0x002..=0x003 => self.greenswap.read_byte(address),
            0x004 => self.dispstat.flags,
            0x005 => self.dispstat.vcount,
            0x006..=0x007 => self.vcount.read_byte(address),
            0x008..=0x009 => self.bgcnt[0].value.read_byte(address),
            0x00A..=0x00B => self.bgcnt[1].value.read_byte(address),
            0x00C..=0x00D => self.bgcnt[2].value.read_byte(address),
            0x00E..=0x00F => self.bgcnt[3].value.read_byte(address),
            0x048..=0x049 => self.winin.value.read_byte(address),
            0x04A..=0x04B => self.winout.value.read_byte(address),
            0x050..=0x051 => self.bldcnt.value.read_byte(address),
            _ => 0, // TODO: open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 0x0400_0000 {
            0x000..=0x001 => self.dispcnt.value.write_byte(address, value),
            0x002..=0x003 => self.greenswap.write_byte(address, value),
            0x004 => self.dispstat.write_flags(value),
            0x005 => self.dispstat.vcount = value,
            0x008..=0x009 => self.bgcnt[0].value.write_byte(address, value),
            0x00A..=0x00B => self.bgcnt[1].value.write_byte(address, value),
            0x00C..=0x00D => self.bgcnt[2].value.write_byte(address, value),
            0x00E..=0x00F => self.bgcnt[3].value.write_byte(address, value),
            0x010..=0x013 => self.bgofs[0].write_byte(address, value),
            0x014..=0x017 => self.bgofs[1].write_byte(address, value),
            0x018..=0x01B => self.bgofs[2].write_byte(address, value),
            0x01C..=0x01F => self.bgofs[3].write_byte(address, value),
            0x020..=0x02F => self.bg2trans.write_byte(address, value),
            0x030..=0x03F => self.bg3trans.write_byte(address, value),
            0x040 => self.winh[0].x1 = value,
            0x041 => self.winh[0].x2 = value,
            0x042 => self.winh[1].x1 = value,
            0x043 => self.winh[1].x2 = value,
            0x044 => self.winv[0].y1 = value,
            0x045 => self.winv[0].y2 = value,
            0x046 => self.winv[1].y1 = value,
            0x047 => self.winv[1].y2 = value,
            0x048..=0x049 => self.winin.value.write_byte(address, value),
            0x04A..=0x04B => self.winout.value.write_byte(address, value),
            0x04C..=0x04D => self.mosaic.value.write_byte(address, value),
            0x050..=0x051 => self.bldcnt.value.write_byte(address, value),
            0x052..=0x053 => self.bldalpha.value.write_byte(address, value),
            0x054..=0x055 => self.bldy.value.write_byte(address, value),
            _ => {}
        }
    }
}
