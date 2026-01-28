#[derive(Debug)]
pub struct FifoBuffer<T, const L: usize> {
    items: [T; L],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const L: usize> FifoBuffer<T, L>
where
    T: Default + Copy,
{
    pub fn push(&mut self, value: T) {
        self.items[self.tail] = value;
        self.tail = (self.tail + 1) % L;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let value = self.items[self.head];
            self.head = (self.head + 1) % L;
            self.len -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.len = 0;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T, const L: usize> Default for FifoBuffer<T, L>
where
    T: Default + Copy,
{
    fn default() -> Self {
        FifoBuffer {
            items: [T::default(); L],
            head: 0,
            tail: 0,
            len: 0,
        }
    }
}
