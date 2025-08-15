use crate::bus::Bus;

const TEST_MEMORY_SIZE: usize = 8196;

#[derive(Debug)]
pub struct TestBus {
    memory: [u8; TEST_MEMORY_SIZE],
}

impl Default for TestBus {
    fn default() -> Self {
        Self {
            memory: [0; TEST_MEMORY_SIZE],
        }
    }
}

impl TestBus {
    pub fn load_program(&mut self, bytes: &[u8]) {
        let slice = &mut self.memory[..bytes.len()];
        slice.copy_from_slice(bytes);
    }
}

impl Bus for TestBus {
    fn read_u32(&self, address: u32) -> u32 {
        let address = address as usize;

        let b1 = self.memory[address];
        let b2 = self.memory[address.wrapping_add(1)];
        let b3 = self.memory[address.wrapping_add(2)];
        let b4 = self.memory[address.wrapping_add(3)];

        u32::from_le_bytes([b1, b2, b3, b4])
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let address = address as usize;
        let bytes = value.to_le_bytes();

        self.memory[address] = bytes[0];
        self.memory[address.wrapping_add(1)] = bytes[1];
        self.memory[address.wrapping_add(2)] = bytes[2];
        self.memory[address.wrapping_add(3)] = bytes[2];
    }
}
