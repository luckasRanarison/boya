use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct Dispstat {
    pub flags: u8,
    pub vcount: u8,
}

impl Dispstat {
    pub const VBLANK: u8 = 0;
    pub const HBLANK: u8 = 1;
    pub const VCOUNT: u8 = 2;
    pub const VBLANK_IRQ: u8 = 3;
    pub const HBLANK_IRQ: u8 = 4;
    pub const VCOUNT_IRQ: u8 = 5;

    pub fn set(&mut self, flag: u8) {
        self.flags.set(flag);
    }

    pub fn clear(&mut self, flag: u8) {
        self.flags.clear(flag);
    }

    pub fn has(&self, flag: u8) -> bool {
        self.flags.has(flag)
    }

    pub fn write_flags(&mut self, value: u8) {
        self.flags = (self.flags & 0b111) | (value & !0b111);
    }
}
