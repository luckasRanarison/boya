pub trait Register {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);
}

impl Register for u16 {
    fn read(&self, address: usize) -> u8 {
        let index = address % 2;
        let bytes = self.to_le_bytes();

        bytes[index]
    }

    fn write(&mut self, address: usize, value: u8) {
        let index = address % 2;
        let mut bytes = self.to_le_bytes();

        bytes[index] = value;
        *self = u16::from_le_bytes(bytes);
    }
}

impl Register for u32 {
    fn read(&self, address: usize) -> u8 {
        let index = address % 4;
        let bytes = self.to_le_bytes();

        bytes[index]
    }

    fn write(&mut self, address: usize, value: u8) {
        let index = address % 4;
        let mut bytes = self.to_le_bytes();

        bytes[index] = value;
        *self = u32::from_le_bytes(bytes);
    }
}
