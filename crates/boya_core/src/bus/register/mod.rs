use crate::bus::{
    Bus,
    register::{
        bgcnt::Bgcnt, dispcnt::Dispcnt, dispstat::Dispstat, dma::Dma, keyinput::KeyInput,
        timer::Timer,
    },
};

pub mod bgcnt;
pub mod dispcnt;
pub mod dispstat;
pub mod dma;
pub mod interrupt;
pub mod keyinput;
pub mod timer;

#[derive(Debug, Default)]
pub struct IORegister {
    /// 0x000: LCD Control (R/W)
    pub dispcnt: Dispcnt,
    /// 0x004: General LCD Status (R/W)
    pub dispstat: Dispstat,
    /// 0x006: Vertical Counter (R)
    pub vcount: u16,
    /// 0x008: Background 0 Control (R/W)
    pub bg0cnt: Bgcnt,
    /// 0x00A: Background 1 Control (R/W)
    pub bg1cnt: Bgcnt,
    /// 0x00C: Background 2 Control (R/W)
    pub bg2cnt: Bgcnt,
    /// 0x00E: Background 3 Control (R/W)
    pub bg3cnt: Bgcnt,
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
    /// 0x0B0: DMA 0 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma0: Dma,
    /// 0x0BC: DMA 1 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma1: Dma,
    /// 0x0C8: DMA 2 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma2: Dma,
    /// 0x0D4: DMA 3 Source Address (W), Destination Address (W), Word Count (W), Control (R/W)
    pub dma3: Dma,
    /// 0x100: Timer 0 Control (R/W)
    pub timer0: Timer,
    /// 0x104: Timer 1 Control (R/W)
    pub timer1: Timer,
    /// 0x108: Timer 2 Control (R/W)
    pub timer2: Timer,
    /// 0x10C: Timer 3 Control (R/W)
    pub timer3: Timer,
    /// 0x130: Key Status (R)
    pub keyinput: KeyInput,
    /// 0x200: Interrupt Enable (R/W)
    pub ie: u16,
    /// 0x202: Interrupt Request Flags (R/W)
    pub irf: u16,
    /// 0x208: Interrupt Master Enable (R/W)
    pub ime: u16,
}

impl Bus for IORegister {
    fn read_byte(&self, address: u32) -> u8 {
        match address {
            0x0400_0000..=0x0400_0001 => self.dispcnt.value.read_byte(address),
            0x0400_0004..=0x0400_0005 => self.dispstat.value.read_byte(address),
            0x0400_0006..=0x0400_0007 => self.vcount.read_byte(address),
            0x0400_0008..=0x0400_0009 => self.bg0cnt.value.read_byte(address),
            0x0400_000A..=0x0400_000B => self.bg1cnt.value.read_byte(address),
            0x0400_000C..=0x0400_000D => self.bg2cnt.value.read_byte(address),
            0x0400_000E..=0x0400_000F => self.bg3cnt.value.read_byte(address),
            0x0400_00BA..=0x0400_00BB => self.dma0.read_byte(address),
            0x0400_00C6..=0x0400_00C7 => self.dma1.read_byte(address),
            0x0400_00D2..=0x0400_00D3 => self.dma2.read_byte(address),
            0x0400_00DE..=0x0400_00DF => self.dma3.read_byte(address),
            0x0400_0100..=0x0400_0103 => self.timer0.read_byte(address),
            0x0400_0104..=0x0400_0107 => self.timer1.read_byte(address),
            0x0400_0108..=0x0400_010B => self.timer2.read_byte(address),
            0x0400_010C..=0x0400_010F => self.timer3.read_byte(address),
            0x0400_0130..=0x0400_0131 => self.keyinput.value.read_byte(address),
            0x0400_0200..=0x0400_0201 => self.ie.read_byte(address),
            0x0400_0202..=0x0400_0203 => self.irf.read_byte(address),
            0x0400_0208..=0x0400_0209 => self.ime.read_byte(address),
            _ => 0, // open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            0x0400_0000..=0x0400_0001 => self.dispcnt.value.write_byte(address, value),
            0x0400_0004..=0x0400_0005 => self.dispstat.value.write_byte(address, value),
            0x0400_0008..=0x0400_0009 => self.bg0cnt.value.write_byte(address, value),
            0x0400_000A..=0x0400_000B => self.bg1cnt.value.write_byte(address, value),
            0x0400_000C..=0x0400_000D => self.bg2cnt.value.write_byte(address, value),
            0x0400_000E..=0x0400_000F => self.bg3cnt.value.write_byte(address, value),
            0x0400_0010..=0x0400_0011 => self.bg0hofs.write_byte(address, value),
            0x0400_0012..=0x0400_0013 => self.bg0vofs.write_byte(address, value),
            0x0400_0014..=0x0400_0015 => self.bg1hofs.write_byte(address, value),
            0x0400_0016..=0x0400_0017 => self.bg1vofs.write_byte(address, value),
            0x0400_0018..=0x0400_0019 => self.bg2hofs.write_byte(address, value),
            0x0400_001A..=0x0400_001B => self.bg2vofs.write_byte(address, value),
            0x0400_001C..=0x0400_001D => self.bg3hofs.write_byte(address, value),
            0x0400_00B0..=0x0400_00BB => self.dma0.write_byte(address, value),
            0x0400_00BC..=0x0400_00C7 => self.dma1.write_byte(address, value),
            0x0400_00C8..=0x0400_00D3 => self.dma2.write_byte(address, value),
            0x0400_00D4..=0x0400_00DF => self.dma3.write_byte(address, value),
            0x0400_0100..=0x0400_0103 => self.timer0.write_byte(address, value),
            0x0400_0104..=0x0400_0107 => self.timer1.write_byte(address, value),
            0x0400_0108..=0x0400_010B => self.timer2.write_byte(address, value),
            0x0400_010C..=0x0400_010F => self.timer3.write_byte(address, value),
            0x0400_0200..=0x0400_0201 => self.ie.write_byte(address, value),
            0x0400_0202..=0x0400_0203 => self.irf.write_byte(address, value),
            0x0400_0208..=0x0400_0209 => self.ime.write_byte(address, value),
            _ => {}
        }
    }
}

impl Bus for u16 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 2;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 2;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u16::from_le_bytes(bytes);
    }
}

impl Bus for u32 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 4;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 4;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u32::from_le_bytes(bytes);
    }
}
