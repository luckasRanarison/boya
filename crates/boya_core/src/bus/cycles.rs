use crate::bus::{EWRAM_PAGE, PALETTE_PAGE, VRAM_PAGE};

pub const CYCLE_LUT_SIZE: usize = 0xF;

#[derive(Debug)]
pub struct CycleLUT {
    pub n16: Box<[u8; CYCLE_LUT_SIZE]>,
    pub n32: Box<[u8; CYCLE_LUT_SIZE]>,
    pub s16: Box<[u8; CYCLE_LUT_SIZE]>,
    pub s32: Box<[u8; CYCLE_LUT_SIZE]>,
}

impl Default for CycleLUT {
    fn default() -> Self {
        let mut n16 = Box::new([1; CYCLE_LUT_SIZE]);
        let mut n32 = Box::new([1; CYCLE_LUT_SIZE]);
        let mut s16 = Box::new([1; CYCLE_LUT_SIZE]);
        let mut s32 = Box::new([1; CYCLE_LUT_SIZE]);

        n16[EWRAM_PAGE] = 3;
        s16[EWRAM_PAGE] = 3;
        n32[EWRAM_PAGE] = 6;
        s32[EWRAM_PAGE] = 6;

        n32[PALETTE_PAGE] = 2;
        s32[PALETTE_PAGE] = 2;

        n32[VRAM_PAGE] = 2;
        s32[VRAM_PAGE] = 2;

        Self { n16, n32, s16, s32 }
    }
}
