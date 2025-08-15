pub trait ExtendedOps: Sized {
    fn wrapping_asr(self, rhs: Self) -> Self;
}

impl ExtendedOps for u32 {
    fn wrapping_asr(self, rhs: Self) -> Self {
        ((self as i32) >> rhs as i32) as u32
    }
}
