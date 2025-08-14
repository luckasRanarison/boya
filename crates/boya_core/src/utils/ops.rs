pub fn asr_u32(lhs: u32, rhs: u32) -> u32 {
    ((lhs as i32) >> rhs as i32) as u32
}
