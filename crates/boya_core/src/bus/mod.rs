pub trait Bus {
    fn read_u8(&self, address: u32) -> u8;
    fn write_u8(&mut self, address: u32, value: u8);

    fn read_u16(&self, address: u32) -> u16 {
        let b1 = self.read_u8(address);
        let b2 = self.read_u8(address + 1);
        u16::from_le_bytes([b1, b2])
    }

    fn read_u32(&self, address: u32) -> u32 {
        let b1 = self.read_u8(address);
        let b2 = self.read_u8(address + 1);
        let b3 = self.read_u8(address + 2);
        let b4 = self.read_u8(address + 3);
        u32::from_le_bytes([b1, b2, b3, b4])
    }

    fn write_u16(&mut self, address: u32, value: u16) {
        let [b1, b2] = value.to_le_bytes();
        self.write_u8(address, b1);
        self.write_u8(address + 1, b2);
    }

    fn write_u32(&mut self, address: u32, value: u32) {
        let [b1, b2, b3, b4] = value.to_le_bytes();
        self.write_u8(address, b1);
        self.write_u8(address + 1, b2);
        self.write_u8(address + 2, b3);
        self.write_u8(address + 3, b4);
    }
}
