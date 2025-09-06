use crate::utils::bitflags::Bitflag;

pub trait ExtendedOps: Sized {
    fn extended_asr(self, rhs: Self) -> Self;
}

impl ExtendedOps for u32 {
    fn extended_asr(self, rhs: Self) -> Self {
        if rhs == 32 {
            if self.has(31) { 0xFFFF_FFFF } else { 0 }
        } else {
            ((self as i32).wrapping_shr(rhs)) as u32
        }
    }
}
