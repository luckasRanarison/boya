use crate::bus::debug::types::{Flag, RegisterEntry, RegisterSize};

impl RegisterEntry {
    const fn dispcnt() -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("BG Mode", 0, 3),
            Flag::new("Display Frame Select", 4, 1),
            Flag::new("H-Blank Interval Free", 5, 1),
            Flag::new("OBJ Character VRAM Maping", 6, 1).map(&[(0, "2D"), (1, "1D")]),
            Flag::new("Forced Blank", 7, 1),
            Flag::new("BG0 enabled", 8, 1),
            Flag::new("BG1 enabled", 9, 1),
            Flag::new("BG2 enabled", 10, 1),
            Flag::new("BG3 enabled", 11, 1),
            Flag::new("OBJ enabled", 12, 1),
            Flag::new("Window 0 Display Flag", 13, 1),
            Flag::new("Window 1 Display Flag", 14, 1),
            Flag::new("OBJ Display Flag", 15, 1),
        ];

        RegisterEntry {
            name: "DISPCNT",
            address: 0x000,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn dispstat() -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("V-Blank", 0, 1),
            Flag::new("H-Blank", 1, 1),
            Flag::new("V-Counter", 2, 1),
            Flag::new("V-Blank IRQ enable", 3, 1),
            Flag::new("H-Blank IRQ enable", 4, 1),
            Flag::new("V-Counter IRQ enable", 5, 1),
            Flag::new("V-Count Setting", 8, 8),
        ];

        RegisterEntry {
            name: "DISPSTAT",
            address: 0x004,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn vcount() -> Self {
        const FLAGS: &[Flag] = &[Flag::new("Scanline", 0, 8)];

        RegisterEntry {
            name: "VCOUNT",
            address: 0x006,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn bgcnt(name: &'static str, address: u32) -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("BG Priority", 0, 2),
            Flag::new("Character Base Block", 2, 2),
            Flag::new("Mosaic", 6, 1),
            Flag::new("Palette", 7, 1).map(&[(0, "16 colors"), (1, "256 colors")]),
            Flag::new("Screen Base Block", 8, 5),
            Flag::new("Display Area Overflow", 13, 1),
            Flag::new("Screen Size", 14, 2).map(&[
                (0, "256x256/128x128"),
                (1, "512x256/256x256"),
                (2, "256x512/512x512"),
                (3, "512x512/1024x1024"),
            ]),
        ];

        Self {
            name,
            address,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn offset(name: &'static str, address: u32) -> Self {
        const FLAGS: &[Flag] = &[Flag::new("Offset", 0, 9)];

        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn float16(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: &[],
        }
    }

    const fn float28(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::Word,
            flags: &[],
        }
    }

    const fn dma_ad(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::Word,
            flags: &[],
        }
    }

    const fn dma_cnt_l(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: &[],
        }
    }

    const fn dma_cnt_h(name: &'static str, address: u32) -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("Dest Address Control", 5, 2).map(&[
                (0, "Increment"),
                (1, "Decrement"),
                (2, "Fixed"),
                (3, "Increment Reload"),
            ]),
            Flag::new("Source Address Control", 7, 2).map(&[
                (0, "Increment"),
                (1, "Decrement"),
                (2, "Fixed"),
                (3, "Prohibited"),
            ]),
            Flag::new("DMA Repeat", 9, 1),
            Flag::new("DMA Transfer Type", 10, 1).map(&[(0, "16-bit"), (1, "32-bit")]),
            Flag::new("Game Pak DRQ", 11, 1),
            Flag::new("DMA Start Timing", 12, 2).map(&[
                (0, "Immediate"),
                (1, "V-Blank"),
                (2, "H-Blank"),
                (3, "Special"),
            ]),
            Flag::new("IRQ Enable", 14, 1),
            Flag::new("DMA Enable", 15, 1),
        ];

        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn tmcnt_l(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: &[],
        }
    }

    const fn tmcnt_h(name: &'static str, address: u32) -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("Prescalar Selection", 0, 2).map(&[
                (0, "F/1"),
                (1, "F/64"),
                (2, "F/256"),
                (3, "F/1024"),
            ]),
            Flag::new("Count-up Timing", 2, 1),
            Flag::new("Timer IRQ enable", 6, 1),
            Flag::new("Timer Start/Stop", 7, 1),
        ];

        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn keyinput() -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("Button A", 0, 1),
            Flag::new("Button B", 1, 1),
            Flag::new("Select", 2, 1),
            Flag::new("Start", 3, 1),
            Flag::new("Right", 4, 1),
            Flag::new("Left", 5, 1),
            Flag::new("Up", 6, 1),
            Flag::new("Down", 7, 1),
            Flag::new("Button R", 8, 1),
            Flag::new("Button L", 9, 1),
        ];

        RegisterEntry {
            name: "KEYINPUT",
            address: 0x130,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn keycnt() -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("Button A", 0, 1),
            Flag::new("Button B", 1, 1),
            Flag::new("Select", 2, 1),
            Flag::new("Start", 3, 1),
            Flag::new("Right", 4, 1),
            Flag::new("Left", 5, 1),
            Flag::new("Up", 6, 1),
            Flag::new("Down", 7, 1),
            Flag::new("Button R", 8, 1),
            Flag::new("Button L", 9, 1),
            Flag::new("IRQ Enable", 14, 1),
            Flag::new("IRQ Condition", 15, 1),
        ];

        RegisterEntry {
            name: "KEYCNT",
            address: 0x132,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn interrupt(name: &'static str, address: u32) -> Self {
        const FLAGS: &[Flag] = &[
            Flag::new("V-Blank", 0, 1),
            Flag::new("H-Blank", 1, 1),
            Flag::new("V-Count", 2, 1),
            Flag::new("Timer 0", 3, 1),
            Flag::new("Timer 1", 4, 1),
            Flag::new("Timer 2", 5, 1),
            Flag::new("Timer 3", 6, 1),
            Flag::new("Serial", 7, 1),
            Flag::new("DMA 0", 8, 1),
            Flag::new("DMA 1", 9, 1),
            Flag::new("DMA 2", 10, 1),
            Flag::new("DMA 3", 11, 1),
            Flag::new("Keypad", 12, 1),
            Flag::new("Game Pak", 13, 1),
        ];

        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }

    const fn ime() -> Self {
        const FLAGS: &[Flag] = &[Flag::new("Disable interrupts", 0, 1)];

        RegisterEntry {
            name: "IME",
            address: 0x208,
            size: RegisterSize::HWord,
            flags: FLAGS,
        }
    }
}

pub const IO_MAP: &[RegisterEntry] = &[
    RegisterEntry::dispcnt(),
    RegisterEntry::dispstat(),
    RegisterEntry::vcount(),
    RegisterEntry::bgcnt("BGCNT0", 0x008),
    RegisterEntry::bgcnt("BGCNT1", 0x00A),
    RegisterEntry::bgcnt("BGCNT2", 0x00C),
    RegisterEntry::bgcnt("BGCNT3", 0x00E),
    RegisterEntry::offset("BG0HOFS", 0x010),
    RegisterEntry::offset("BG0VOFS", 0x012),
    RegisterEntry::offset("BG1HOFS", 0x014),
    RegisterEntry::offset("BG1VOFS", 0x016),
    RegisterEntry::offset("BG2HOFS", 0x018),
    RegisterEntry::offset("BG2VOFS", 0x01A),
    RegisterEntry::offset("BG3HOFS", 0x01C),
    RegisterEntry::offset("BG3VOFS", 0x01E),
    RegisterEntry::float16("BG2PA", 0x020),
    RegisterEntry::float16("BG2PB", 0x022),
    RegisterEntry::float16("BG2PC", 0x024),
    RegisterEntry::float16("BG2PD", 0x026),
    RegisterEntry::float28("BG2X", 0x028),
    RegisterEntry::float28("BG2Y", 0x02C),
    RegisterEntry::float16("BG3PA", 0x030),
    RegisterEntry::float16("BG3PB", 0x032),
    RegisterEntry::float16("BG3PC", 0x034),
    RegisterEntry::float16("BG3PD", 0x036),
    RegisterEntry::float28("BG3X", 0x038),
    RegisterEntry::float28("BG3Y", 0x03C),
    RegisterEntry::dma_ad("DMA0SAD", 0x0B0),
    RegisterEntry::dma_ad("DMA0DAD", 0x0B4),
    RegisterEntry::dma_cnt_l("DMA0CNT_L", 0x0B8),
    RegisterEntry::dma_cnt_h("DMA0CNT_H", 0x0BA),
    RegisterEntry::dma_ad("DMA1SAD", 0x0BC),
    RegisterEntry::dma_ad("DMA1DAD", 0x0C0),
    RegisterEntry::dma_cnt_l("DMA1CNT_L", 0x0C4),
    RegisterEntry::dma_cnt_h("DMA1CNT_H", 0x0C6),
    RegisterEntry::dma_ad("DMA2SAD", 0x0C8),
    RegisterEntry::dma_ad("DMA2DAD", 0x0CC),
    RegisterEntry::dma_cnt_l("DMA2CNT_L", 0x0D0),
    RegisterEntry::dma_cnt_h("DMA2CNT_H", 0x0D2),
    RegisterEntry::dma_ad("DMA3SAD", 0x0D4),
    RegisterEntry::dma_ad("DMA3DAD", 0x0D8),
    RegisterEntry::dma_cnt_l("DMA3CNT_L", 0x0DC),
    RegisterEntry::dma_cnt_h("DMA3CNT_H", 0x0DE),
    RegisterEntry::tmcnt_l("TM0CNT_L", 0x100),
    RegisterEntry::tmcnt_h("TM0CNT_H", 0x102),
    RegisterEntry::tmcnt_l("TM1CNT_L", 0x104),
    RegisterEntry::tmcnt_h("TM1CNT_H", 0x106),
    RegisterEntry::tmcnt_l("TM2CNT_L", 0x108),
    RegisterEntry::tmcnt_h("TM2CNT_H", 0x10A),
    RegisterEntry::tmcnt_l("TM3CNT_L", 0x10C),
    RegisterEntry::tmcnt_h("TM3CNT_H", 0x10E),
    RegisterEntry::keyinput(),
    RegisterEntry::keycnt(),
    RegisterEntry::interrupt("IE", 0x200),
    RegisterEntry::interrupt("IF", 0x202),
    RegisterEntry::ime(),
];
