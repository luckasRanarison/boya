pub mod interrupt_flags {
    pub const VBLANK: u16 = 0;
    pub const HBLANK: u16 = 1;
    pub const VCOUNT_MATCH: u16 = 2;
    pub const TIMER0: u16 = 3;
    pub const TIMER1: u16 = 4;
    pub const TIMER2: u16 = 5;
    pub const TIMER3: u16 = 6;
    pub const SERIAL: u16 = 7;
    pub const DMA0: u16 = 8;
    pub const DMA1: u16 = 9;
    pub const DMA2: u16 = 10;
    pub const DMA3: u16 = 11;
    pub const KEYPAD: u16 = 12;
    pub const GAMEPAK: u16 = 13;
}
