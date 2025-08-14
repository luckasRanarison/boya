pub trait Bus {
    fn read_u32(&self, address: u32) -> u32;
    fn write_u32(&mut self, address: u32, value: u32);
}
