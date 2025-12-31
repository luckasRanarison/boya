use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct Dispstat {
    pub value: u16,
}

impl Dispstat {
    pub const VBLANK: u16 = 0;
    pub const HBLANK: u16 = 1;
    pub const VCOUNT: u16 = 2;
    pub const VBLANK_IRQ: u16 = 3;
    pub const HBLANK_IRQ: u16 = 4;
    pub const VCOUNT_IRQ: u16 = 5;

    pub fn set(&mut self, flag: u16) {
        self.value.set(flag);
    }

    pub fn clear(&mut self, flag: u16) {
        self.value.clear(flag);
    }

    pub fn has(&self, flag: u16) -> bool {
        self.value.has(flag)
    }

    pub fn get_vcount(&self) -> u16 {
        self.value.get_bits(8, 15)
    }
}
