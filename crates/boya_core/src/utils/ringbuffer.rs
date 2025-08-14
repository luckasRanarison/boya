#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    items: [T; N],
    r_ptr: usize,
    w_ptr: usize,
    size: usize,
}

impl<T, const N: usize> RingBuffer<T, N>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            items: [T::default(); N],
            r_ptr: 0,
            w_ptr: 0,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items[self.w_ptr] = value;
        self.w_ptr = (self.w_ptr + 1) % N;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let value = self.items[self.r_ptr];
        self.r_ptr = (self.r_ptr + 1) % N;
        self.size -= 1;
        Some(value)
    }

    pub fn dequeue_unchecked(&mut self) -> T {
        self.dequeue().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_ringbuffer() {
        let mut rb = RingBuffer::<u32, 2>::new();

        rb.enqueue(1);
        rb.enqueue(2);

        assert_eq!(rb.dequeue(), Some(1));
        assert_eq!(rb.dequeue(), Some(2));
    }
}
