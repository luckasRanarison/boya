#[derive(Debug, Default)]
pub struct IrqRequestFlag {
    pub value: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    VBlank,
    HBlank,
    VCount,
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Serial,
    Dma0,
    Dma1,
    Dma2,
    Dma3,
    Keypad,
    Gamepak,
}
