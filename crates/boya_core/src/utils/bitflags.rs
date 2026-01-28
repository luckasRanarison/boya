use std::ops::*;

pub trait Bitflag: Sized {
    fn get(self, bit: Self) -> Self;
    fn has(self, bit: Self) -> bool;
    fn set(&mut self, bit: Self);
    fn clear(&mut self, bit: Self);
    fn take(&mut self, bit: Self) -> bool;

    fn set_bits(&mut self, start: Self, end: Self, value: Self);
    fn get_bits(self, start: Self, end: Self) -> Self;

    fn get_u8(self, bit: Self) -> u8;
    fn get_bits_u8(self, start: Self, end: Self) -> u8;

    fn update(&mut self, bit: Self, cond: bool) {
        if cond { self.set(bit) } else { self.clear(bit) }
    }
}

impl<T> Bitflag for T
where
    T: From<u8>
        + TryInto<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Not<Output = T>
        + BitAndAssign
        + BitOrAssign
        + PartialEq
        + Copy,
    <T as TryInto<u8>>::Error: std::fmt::Debug,
{
    #[inline(always)]
    fn get(self, bit: T) -> T {
        (self >> bit) & T::from(1)
    }

    #[inline(always)]
    fn get_u8(self, bit: T) -> u8 {
        self.get(bit).try_into().unwrap()
    }

    #[inline(always)]
    fn set(&mut self, bit: T) {
        *self |= T::from(1) << bit;
    }

    #[inline(always)]
    fn clear(&mut self, bit: T) {
        *self &= !(T::from(1) << bit);
    }

    #[inline(always)]
    fn take(&mut self, bit: Self) -> bool {
        let res = !(T::from(1) << bit);
        *self &= res;
        res != T::from(0)
    }

    #[inline(always)]
    fn set_bits(&mut self, start: T, end: T, value: T) {
        let one = T::from(1);
        let mask = ((one << (end - start + one)) - one) << start;
        *self = (*self & !mask) | ((value << start) & mask);
    }

    #[inline(always)]
    fn get_bits(self, start: T, end: T) -> T {
        let one = T::from(1);
        let mask = (one << (end - start + one)) - one;
        (self >> start) & mask
    }

    #[inline(always)]
    fn get_bits_u8(self, start: T, end: T) -> u8 {
        self.get_bits(start, end).try_into().unwrap()
    }

    #[inline(always)]
    fn has(self, bit: T) -> bool {
        self.get(bit) == T::from(1)
    }
}

pub trait BitArray {
    fn to_bit_array<const N: usize>(self, start: usize) -> [u8; N];
}

impl<T> BitArray for T
where
    T: BitAnd<Output = T> + Shr<Output = T> + ShrAssign + PartialEq + From<u8> + Copy,
{
    fn to_bit_array<const N: usize>(self, offset: usize) -> [u8; N] {
        let mut buffer = [0; N];

        let shiftd = self >> T::from(offset as u8);
        let bits = shiftd.iter_lsb().take(N);

        for (idx, bit) in bits {
            buffer[N - 1 - idx] = bit;
        }

        buffer
    }
}

pub trait BitIter: Copy {
    fn iter_lsb(self) -> impl Iterator<Item = (usize, u8)>;
}

impl<T> BitIter for T
where
    T: BitAnd<Output = T> + Shr<Output = T> + ShrAssign + PartialEq + From<u8> + Copy,
{
    fn iter_lsb(self) -> impl Iterator<Item = (usize, u8)> {
        let one = T::from(1);
        let range = 0..size_of::<T>() * 8;
        let mut shifted = self;

        range.map(move |index| {
            let bit = (shifted & one) == one;
            shifted >>= one;
            (index, bit as u8)
        })
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
        let value = 0b1010_u8;
        let arr = value.to_bit_array::<3>(1);

        assert_eq!(arr, [1, 0, 1]);
    }

    #[test]
    fn test_bit_iter() {
        let value = 0b00111010_u8;
        let bits_lsb = value.iter_lsb().collect::<Vec<_>>();

        assert_eq!(
            &bits_lsb,
            &[
                (0, 0),
                (1, 1),
                (2, 0),
                (3, 1),
                (4, 1),
                (5, 1),
                (6, 0),
                (7, 0)
            ]
        );
    }
}
