#[derive(Debug)]
pub enum InterruptFlag {
    Vblank,
    Hblank,
    VCountMatch,
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
