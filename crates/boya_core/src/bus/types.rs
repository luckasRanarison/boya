#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Byte = 1,
    HWord = 2,
    Word = 4,
}

#[derive(Debug)]
pub enum MemoryAccess {
    Seq,
    NonSeq,
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub width: DataType,
    pub waitstate: WaitState,
}

#[derive(Debug, Default)]
pub struct WaitState {
    pub n: u8,
    pub s: u8,
}
