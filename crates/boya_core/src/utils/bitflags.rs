use std::ops::{BitAnd, Shr, ShrAssign};

pub trait Bitflag: Sized {
    fn get(self, bit: Self) -> Self;
    fn has(self, bit: Self) -> bool;
    fn set(&mut self, bit: Self);
    fn clear(&mut self, bit: Self);

    fn set_bits(&mut self, start: Self, end: Self, value: Self);
    fn get_bits(self, start: Self, end: Self) -> Self;

    fn get_u8(self, bit: Self) -> u8;
    fn get_bits_u8(self, start: Self, end: Self) -> u8;

    fn update(&mut self, bit: Self, cond: bool) {
        if cond { self.set(bit) } else { self.clear(bit) }
    }
}

impl Bitflag for u32 {
    #[inline(always)]
    fn get(self, bit: u32) -> u32 {
        (self >> bit) & 1
    }

    #[inline(always)]
    fn get_u8(self, bit: u32) -> u8 {
        self.get(bit) as u8
    }

    #[inline(always)]
    fn set(&mut self, bit: u32) {
        *self |= 1 << bit;
    }

    #[inline(always)]
    fn clear(&mut self, bit: u32) {
        *self &= !(1 << bit);
    }

    #[inline(always)]
    fn set_bits(&mut self, start: u32, end: u32, value: u32) {
        let mask = ((1 << (end - start + 1)) - 1) << start;
        *self = (*self & !mask) | ((value << start) & mask);
    }

    #[inline(always)]
    fn get_bits(self, start: u32, end: u32) -> u32 {
        (self >> start) & ((1 << (end - start + 1)) - 1)
    }

    #[inline(always)]
    fn get_bits_u8(self, start: u32, end: u32) -> u8 {
        self.get_bits(start, end) as u8
    }

    #[inline(always)]
    fn has(self, bit: u32) -> bool {
        self.get(bit) == 1
    }
}

impl Bitflag for u16 {
    #[inline(always)]
    fn get(self, bit: u16) -> u16 {
        (self >> bit) & 1
    }

    #[inline(always)]
    fn get_u8(self, bit: u16) -> u8 {
        self.get(bit) as u8
    }

    #[inline(always)]
    fn set(&mut self, bit: u16) {
        *self |= 1 << bit;
    }

    #[inline(always)]
    fn clear(&mut self, bit: u16) {
        *self &= !(1 << bit);
    }

    #[inline(always)]
    fn set_bits(&mut self, start: u16, end: u16, value: u16) {
        let mask = ((1 << (end - start + 1)) - 1) << start;
        *self = (*self & !mask) | ((value << start) & mask);
    }

    #[inline(always)]
    fn get_bits(self, start: u16, end: u16) -> u16 {
        (self >> start) & ((1 << (end - start + 1)) - 1)
    }

    #[inline(always)]
    fn get_bits_u8(self, start: u16, end: u16) -> u8 {
        self.get_bits(start, end) as u8
    }

    #[inline(always)]
    fn has(self, bit: u16) -> bool {
        self.get(bit) == 1
    }
}

pub trait BitArray {
    fn to_bit_array<const N: usize>(self, start: u8) -> [u8; N];
}

impl<T> BitArray for T
where
    T: BitAnd<Output = T> + Shr<Output = T> + ShrAssign + PartialEq + From<u8> + Copy,
{
    fn to_bit_array<const N: usize>(self, start: u8) -> [u8; N] {
        let one = T::from(1);
        let mut buffer = [0; N];
        let mut shifted = self >> T::from(start);

        for i in 0..N {
            buffer[N - 1 - i] = ((shifted & one) == one) as u8;
            shifted >>= one;
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_range() {
        let value = 0b101101_u32;
        let range = value.get_bits(2, 5);

        assert_eq!(range, 0b1011);
    }

    #[test]
    fn test_set_range() {
        let mut value = 0b101101_u16;
        value.set_bits(2, 5, 0b0100);

        assert_eq!(value, 0b010001);
    }

    #[test]
    fn test_bit_array() {
        let value = 0b1010;
        let arr = value.to_bit_array::<3>(1);

        assert_eq!(arr, [1, 0, 1]);
    }
}
