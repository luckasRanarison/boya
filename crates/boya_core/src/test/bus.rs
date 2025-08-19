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
    pub fn load_program(&mut self, bytes: &[u8], offset: usize) {
        let slice = &mut self.memory[offset..bytes.len() + offset];
        slice.copy_from_slice(bytes);
    }
}

impl Bus for TestBus {
    fn read_byte(&self, address: u32) -> u8 {
        self.memory[(address as usize) % TEST_MEMORY_SIZE]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        self.memory[(address as usize) % TEST_MEMORY_SIZE] = value;
    }
}
