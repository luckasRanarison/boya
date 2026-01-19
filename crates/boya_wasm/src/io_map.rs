use crate::types::{Flag, IOMap, RegisterEntry, RegisterSize};

impl RegisterEntry {
    fn dispcnt() -> Self {
        RegisterEntry {
            name: "DISPCNT",
            address: 0x000,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("BG Mode", 0, Some(2)),
                Flag::new("Display Frame Select", 4, None),
                Flag::new("H-Blank Interval Free", 5, None),
                Flag::new("OBJ Character VRAM Maping", 6, None),
                Flag::new("Forced Blank", 7, None),
                Flag::new("BG0 enabled", 8, None),
                Flag::new("BG1 enabled", 9, None),
                Flag::new("BG2 enabled", 10, None),
                Flag::new("BG3 enabled", 11, None),
                Flag::new("OBJ enabled", 12, None),
                Flag::new("Window 0 Display Flag", 13, None),
                Flag::new("Window 1 Display Flag", 14, None),
                Flag::new("OBJ Display Flag", 15, None),
            ],
        }
    }

    fn dispstat() -> Self {
        RegisterEntry {
            name: "DISPSTAT",
            address: 0x004,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("V-Blank", 0, None),
                Flag::new("H-Blank", 1, None),
                Flag::new("V-Counter", 2, None),
                Flag::new("V-Blank IRQ enable", 3, None),
                Flag::new("H-Blank IRQ enable", 4, None),
                Flag::new("V-Counter IRQ enable", 5, None),
                Flag::new("V-Count Setting", 8, Some(15)),
            ],
        }
    }

    fn vcount() -> Self {
        RegisterEntry {
            name: "VCOUNT",
            address: 0x006,
            size: RegisterSize::HWord,
            flags: vec![Flag::new("Scanline", 0, Some(7))],
        }
    }

    fn bgcnt(name: &'static str, address: u32) -> Self {
        Self {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("BG Priority", 0, Some(1)),
                Flag::new("Character Base Block", 2, Some(3)),
                Flag::new("Mosaic", 6, None),
                Flag::new("Palette", 7, None),
                Flag::new("Screen Base Block", 8, Some(12)),
                Flag::new("Display Area Overflow", 13, None),
                Flag::new("Screen Size", 14, Some(15)),
            ],
        }
    }

    fn offset(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![Flag::new("Offset", 0, Some(8))],
        }
    }

    fn dma_ad(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::Word,
            flags: vec![],
        }
    }

    fn dma_cnt_l(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![],
        }
    }

    fn dma_cnt_h(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("Dest Address Control", 5, Some(6)),
                Flag::new("Source Address Control", 7, Some(8)),
                Flag::new("DMA Repeat", 9, None),
                Flag::new("DMA Transfer Type", 10, None),
                Flag::new("Game Pak DRQ", 11, None),
                Flag::new("DMA Start Timing", 12, Some(13)),
                Flag::new("IRQ Enable", 14, None),
                Flag::new("DMA Enable", 15, None),
            ],
        }
    }

    fn tmcnt_l(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![],
        }
    }

    fn tmcnt_h(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("Prescalar Selection", 0, Some(1)),
                Flag::new("Count-up Timing", 2, None),
                Flag::new("Timer IRQ enable", 6, None),
                Flag::new("Timer Start/Stop", 7, None),
            ],
        }
    }

    fn keyinput() -> Self {
        RegisterEntry {
            name: "KEYINPUT",
            address: 0x130,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("Button A", 0, None),
                Flag::new("Button B", 1, None),
                Flag::new("Select", 2, None),
                Flag::new("Start", 3, None),
                Flag::new("Right", 4, None),
                Flag::new("Left", 5, None),
                Flag::new("Up", 6, None),
                Flag::new("Down", 7, None),
                Flag::new("Button R", 8, None),
                Flag::new("Button L", 9, None),
            ],
        }
    }

    fn keycnt() -> Self {
        RegisterEntry {
            name: "KEYCNT",
            address: 0x132,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("Button A", 0, None),
                Flag::new("Button B", 1, None),
                Flag::new("Select", 2, None),
                Flag::new("Start", 3, None),
                Flag::new("Right", 4, None),
                Flag::new("Left", 5, None),
                Flag::new("Up", 6, None),
                Flag::new("Down", 7, None),
                Flag::new("Button R", 8, None),
                Flag::new("Button L", 9, None),
                Flag::new("IRQ Enable", 14, None),
                Flag::new("IRQ Condition", 15, None),
            ],
        }
    }

    fn interrupt(name: &'static str, address: u32) -> Self {
        RegisterEntry {
            name,
            address,
            size: RegisterSize::HWord,
            flags: vec![
                Flag::new("V-Blank", 0, None),
                Flag::new("H-Blank", 1, None),
                Flag::new("V-Count", 2, None),
                Flag::new("Timer 0", 3, None),
                Flag::new("Timer 1", 4, None),
                Flag::new("Timer 2", 5, None),
                Flag::new("Timer 3", 6, None),
                Flag::new("Serial", 7, None),
                Flag::new("DMA 0", 8, None),
                Flag::new("DMA 1", 9, None),
                Flag::new("DMA 2", 10, None),
                Flag::new("DMA 3", 11, None),
                Flag::new("Keypad", 12, None),
                Flag::new("Game Pak", 13, None),
            ],
        }
    }

    fn ime() -> Self {
        RegisterEntry {
            name: "IME",
            address: 0x208,
            size: RegisterSize::HWord,
            flags: vec![Flag::new("Disable interrupts", 0, None)],
        }
    }
}

impl Default for IOMap {
    fn default() -> Self {
        IOMap(vec![
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
        ])
    }
}
