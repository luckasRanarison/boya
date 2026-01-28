use crate::bus::debug::types::{Flag, RegisterEntry, RegisterSize};

impl RegisterEntry {
    const fn dispcnt() -> Self {
        RegisterEntry {
            name: "DISPCNT",
            address: 0x000,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("BG Mode", 0, 3),
                    Flag::unused(3, 1),
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
                ]
            },
        }
    }

    const fn dispstat() -> Self {
        RegisterEntry {
            name: "DISPSTAT",
            address: 0x004,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("V-Blank", 0, 1),
                    Flag::new("H-Blank", 1, 1),
                    Flag::new("V-Counter", 2, 1),
                    Flag::new("V-Blank IRQ enable", 3, 1),
                    Flag::new("H-Blank IRQ enable", 4, 1),
                    Flag::new("V-Counter IRQ enable", 5, 1),
                    Flag::unused(6, 2),
                    Flag::new("V-Count Setting", 8, 8),
                ]
            },
        }
    }

    const fn vcount() -> Self {
        RegisterEntry {
            name: "VCOUNT",
            address: 0x006,
            size: RegisterSize::HWord,
            flags: const { &[Flag::new("Scanline", 0, 8), Flag::unused(8, 8)] },
        }
    }

    const fn bgcnt(name: &'static str, address: u32) -> Self {
        Self {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("BG Priority", 0, 2),
                    Flag::new("Character Base Block", 2, 2),
                    Flag::unused(4, 2),
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
                ]
            },
        }
    }

    const fn offset(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const { &[Flag::new("value", 0, 9), Flag::unused(9, 7)] },
        }
    }

    const fn float16(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("Fractional Portion", 0, 8),
                    Flag::new("Integer Portion", 8, 7),
                    Flag::new("Sign", 15, 1),
                ]
            },
        }
    }

    const fn float28(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::Word,
            flags: const {
                &[
                    Flag::new("Fractional Portion", 0, 8),
                    Flag::new("Integer Portion", 8, 19),
                    Flag::new("Sign", 27, 1),
                    Flag::unused(28, 4),
                ]
            },
        }
    }

    const fn soundcnt_l() -> Self {
        RegisterEntry {
            name: "SOUNDCNT_L",
            address: 0x080,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("Sound 1-4 Master Volume RIGHT", 0, 3),
                    Flag::unused(3, 1),
                    Flag::new("Sound 1-4 Master Volume LEFT", 4, 3),
                    Flag::unused(7, 1),
                    Flag::new("Sound 1-4 Enable Flags LEFT", 8, 4),
                    Flag::new("Sound 1-4 Enable Flags RIGHT", 12, 4),
                ]
            },
        }
    }

    const fn soundcnt_h() -> Self {
        RegisterEntry {
            name: "SOUNDCNT_H",
            address: 0x082,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("Sound 1-4 Volume", 0, 2).map(&[
                        (0, "25%"),
                        (1, "50%"),
                        (2, "100%"),
                        (3, "Prohibited"),
                    ]),
                    Flag::new("Sound A Volume", 2, 1).map(&[(0, "50%"), (1, "100%")]),
                    Flag::new("Sound B Volume", 3, 1).map(&[(0, "50%"), (1, "100%")]),
                    Flag::unused(4, 4),
                    Flag::new("Sound A Enable RIGHT", 8, 1),
                    Flag::new("Sound A Enable LEFT", 9, 1),
                    Flag::new("Sound A Timer Select", 10, 1),
                    Flag::new("Sound A Reset FIFO", 11, 1),
                    Flag::new("Sound B Enable RIGHT", 12, 1),
                    Flag::new("Sound B Enable LEFT", 13, 1),
                    Flag::new("Sound B Timer Select", 14, 1),
                    Flag::new("Sound B Reset FIFO", 15, 1),
                ]
            },
        }
    }

    const fn sg_bias() -> Self {
        RegisterEntry {
            name: "SG_BIAS",
            address: 0x88,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("Bias Level", 0, 10),
                    Flag::unused(10, 4),
                    Flag::new("Amplitude Resolution Cycle", 14, 2).map(&[
                        (0, "9bit / 32.768kHz"),
                        (1, "8bit / 65.536kHz"),
                        (2, "7bit / 131.072kHz"),
                        (3, "6bit / 262.144kHz"),
                    ]),
                ]
            },
        }
    }

    const fn dma_ad(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::Word,
            flags: const { &[Flag::new("value", 0, 28), Flag::unused(28, 4)] },
        }
    }

    const fn dma_cnt_l<const B: u8>(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                if B == 16 {
                    &[Flag::new("value", 0, B)]
                } else {
                    &[Flag::new("value", 0, B), Flag::unused(B, 16 - B)]
                }
            },
        }
    }

    const fn dma_cnt_h(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::unused(0, 5),
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
                ]
            },
        }
    }

    const fn tmcnt_l(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const { &[Flag::new("value", 0, 16)] },
        }
    }

    const fn tmcnt_h(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                &[
                    Flag::new("Prescalar Selection", 0, 2).map(&[
                        (0, "F/1"),
                        (1, "F/64"),
                        (2, "F/256"),
                        (3, "F/1024"),
                    ]),
                    Flag::new("Count-up Timing", 2, 1),
                    Flag::unused(3, 3),
                    Flag::new("Timer IRQ enable", 6, 1),
                    Flag::new("Timer Start/Stop", 7, 1),
                    Flag::unused(8, 8),
                ]
            },
        }
    }

    const fn keyinput() -> Self {
        RegisterEntry {
            name: "KEYINPUT",
            address: 0x130,
            size: RegisterSize::HWord,
            flags: const {
                &[
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
                    Flag::unused(10, 6),
                ]
            },
        }
    }

    const fn keycnt() -> Self {
        RegisterEntry {
            name: "KEYCNT",
            address: 0x132,
            size: RegisterSize::HWord,
            flags: const {
                &[
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
                    Flag::unused(10, 4),
                    Flag::new("IRQ Enable", 14, 1),
                    Flag::new("IRQ Condition", 15, 1),
                ]
            },
        }
    }

    const fn interrupt(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: const {
                &[
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
                    Flag::unused(14, 2),
                ]
            },
        }
    }

    const fn ime() -> Self {
        RegisterEntry {
            name: "IME",
            address: 0x208,
            size: RegisterSize::HWord,
            flags: const { &[Flag::new("Disable interrupts", 0, 1), Flag::unused(1, 15)] },
        }
    }

    const fn haltcnt_l() -> Self {
        RegisterEntry {
            name: "HALTCNT_L",
            address: 0x300,
            size: RegisterSize::Byte,
            flags: const { &[Flag::new("First Boot Flag", 0, 1), Flag::unused(1, 7)] },
        }
    }

    const fn haltcnt_h() -> Self {
        RegisterEntry {
            name: "HALTCNT_H",
            address: 0x301,
            size: RegisterSize::Byte,
            flags: const { &[Flag::unused(1, 7), Flag::new("Power Down Mode", 7, 1)] },
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
    RegisterEntry::soundcnt_l(),
    RegisterEntry::soundcnt_h(),
    RegisterEntry::sg_bias(),
    RegisterEntry::dma_ad("DMA0SAD", 0x0B0),
    RegisterEntry::dma_ad("DMA0DAD", 0x0B4),
    RegisterEntry::dma_cnt_l::<14>("DMA0CNT_L", 0x0B8),
    RegisterEntry::dma_cnt_h("DMA0CNT_H", 0x0BA),
    RegisterEntry::dma_ad("DMA1SAD", 0x0BC),
    RegisterEntry::dma_ad("DMA1DAD", 0x0C0),
    RegisterEntry::dma_cnt_l::<14>("DMA1CNT_L", 0x0C4),
    RegisterEntry::dma_cnt_h("DMA1CNT_H", 0x0C6),
    RegisterEntry::dma_ad("DMA2SAD", 0x0C8),
    RegisterEntry::dma_ad("DMA2DAD", 0x0CC),
    RegisterEntry::dma_cnt_l::<14>("DMA2CNT_L", 0x0D0),
    RegisterEntry::dma_cnt_h("DMA2CNT_H", 0x0D2),
    RegisterEntry::dma_ad("DMA3SAD", 0x0D4),
    RegisterEntry::dma_ad("DMA3DAD", 0x0D8),
    RegisterEntry::dma_cnt_l::<16>("DMA3CNT_L", 0x0DC),
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
    RegisterEntry::haltcnt_l(),
    RegisterEntry::haltcnt_h(),
];
