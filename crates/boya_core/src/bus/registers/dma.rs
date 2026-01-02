use crate::{
    bus::{
        Bus,
        types::{DataType, Interrupt},
    },
    utils::bitflags::Bitflag,
};

#[derive(Debug, Default, Clone)]
pub struct Dma {
    pub sad: u32,
    pub dad: u32,
    pub cnt_l: u16,
    pub cnt_h: u16,
    pub channel: DmaChannel,
}

impl Dma {
    pub fn new(channel: DmaChannel) -> Self {
        Self {
            channel,
            ..Default::default()
        }
    }

    pub fn dst_addr_control(&self) -> DmaAddressControl {
        match self.cnt_h.get_bits(5, 6) {
            0 => DmaAddressControl::Increment,
            1 => DmaAddressControl::Decrement,
            2 => DmaAddressControl::Fixed,
            _ => DmaAddressControl::IncrementReload,
        }
    }

    pub fn src_addr_control(&self) -> DmaAddressControl {
        match self.cnt_h.get_bits(7, 8) {
            0 => DmaAddressControl::Increment,
            1 => DmaAddressControl::Decrement,
            2 => DmaAddressControl::Fixed,
            _ => unreachable!("invalid address control, prohibited"),
        }
    }

    pub fn repeat(&self) -> bool {
        self.cnt_h.has(7)
    }

    pub fn transfer_type(&self) -> DataType {
        match self.cnt_h.get(10) {
            0 => DataType::HWord,
            _ => DataType::Word,
        }
    }

    pub fn start_timing(&self) -> DmaStartTiming {
        match self.cnt_h.get_bits(12, 13) {
            0 => DmaStartTiming::Immediate,
            1 => DmaStartTiming::VBlank,
            2 => DmaStartTiming::HBlank,
            _ => DmaStartTiming::Special,
        }
    }

    pub fn irq_enable(&self) -> bool {
        self.cnt_h.has(14)
    }

    pub fn dma_enable(&self) -> bool {
        self.cnt_h.has(15)
    }

    pub fn disable(&mut self) {
        self.cnt_h.clear(15);
    }

    pub fn transfer_len(&self) -> u32 {
        match (self.cnt_l, self.channel) {
            (0, DmaChannel::Dma3) => 0x10000,
            (_, DmaChannel::Dma3) => self.cnt_l as u32,
            (0, _) => 0x4000,
            (_, _) => self.cnt_l as u32 & 0x3FFF,
        }
    }

    pub fn special_timing(&self) -> DmaSpecialTiming {
        self.channel.special_timing()
    }
}

impl Bus for Dma {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 12 {
            10..=11 => self.cnt_h.read_byte(address),
            _ => 0, // write-only
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 12 {
            0..=3 => self.sad.write_byte(address, value),
            4..=7 => self.dad.write_byte(address, value),
            8..=9 => self.cnt_l.write_byte(address, value),
            _ => self.cnt_h.write_byte(address, value),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DmaChannel {
    #[default]
    Dma0,
    Dma1,
    Dma2,
    Dma3,
}

impl DmaChannel {
    pub fn special_timing(self) -> DmaSpecialTiming {
        match self {
            DmaChannel::Dma0 => DmaSpecialTiming::None,
            DmaChannel::Dma1 => DmaSpecialTiming::FifoA,
            DmaChannel::Dma2 => DmaSpecialTiming::FifoB,
            DmaChannel::Dma3 => DmaSpecialTiming::VideoCapture,
        }
    }
}

impl From<DmaChannel> for Interrupt {
    fn from(value: DmaChannel) -> Self {
        match value {
            DmaChannel::Dma0 => Interrupt::Dma0,
            DmaChannel::Dma1 => Interrupt::Dma1,
            DmaChannel::Dma2 => Interrupt::Dma2,
            DmaChannel::Dma3 => Interrupt::Dma3,
        }
    }
}

#[derive(Debug)]
pub enum DmaSpecialTiming {
    None,
    FifoA,
    FifoB,
    VideoCapture,
}

#[derive(Debug)]
pub enum DmaStartTiming {
    Immediate,
    VBlank,
    HBlank,
    Special,
}

#[derive(Debug)]
pub enum DmaAddressControl {
    Increment,
    Decrement,
    Fixed,
    IncrementReload,
}

#[cfg(test)]
mod tests {
    use crate::{bus::types::Interrupt, test::GbaTestBuilder};

    #[test]
    fn test_dma() {
        let asm = r"
            _setup:
                B       start

            _chunk_to_copy:
                dw      0x5010
                dw      0x10FF
                dw      0x2050
                dw      0xA030

            start:
                ; set source address to _chunk_to_copy
                MOV     R0, #0x0400_0000
                MOV     R1, #0xB0
                ADR     R2, _chunk_to_copy
                STR     R2, [R0, R1]

                ; set destination address to IWRAM
                MOV     R1, #0xB4
                MOV     R2, #0x0300_0000
                STR     R2, [R0, R1]

                ; set transfer length to 8
                MOV     R1, #0xB8
                MOV     R2, #0x8
                STRH    R2, [R0, R1]

                ; Start DMA (16bit, immediate)
                MOV     R1, #0xBA
                MOV     R2, #0x0
                ORR     R2, #(1 shl 14) ; set irq enable
                ORR     R2, #(1 shl 15) ; set dma enable
                STRH    R2, [R0, R1]
        ";

        let expected_chunks = vec![0x5010, 0x10FF, 0x2050, 0xA030]
            .into_iter()
            .map(|c: u32| c.to_le_bytes())
            .flatten()
            .collect::<Vec<_>>();

        GbaTestBuilder::new()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.io.enable_master_irq();
                cpu.bus.io.enable_irq(Interrupt::Dma0);
            })
            .assert_fn(move |cpu| {
                let dma0 = &cpu.bus.io.dma0;

                assert_eq!(0x0800_0004, dma0.sad, "DMA0 source address");
                assert_eq!(0x0300_0000, dma0.dad, "DMA0 destination address");
                assert_eq!(8, dma0.transfer_len(), "DMA0 transfer length");
                assert_eq!(&cpu.bus.iwram[..16], &expected_chunks);
                assert!(!dma0.dma_enable(), "DMA0 should be disabled");
                assert!(cpu.bus.io.has_irq(Interrupt::Dma0), "DMA0 pending irq");
            })
            .assert_cycles([
                20, // B   (2S + 1N)
                6,  // MOV (1S)
                6,  // MOV (1S)
                6,  // SUB (1S)
                9,  // STR (2N)
                6,  // MOV (1S)
                6,  // MOV (1S)
                9,  // STR (2N)
                6,  // MOV (1S)
                6,  // MOV (1S)
                9,  // STR (2N)
                6,  // MOV (1S)
                6,  // MOV (1S)
                6,  // ORR (1S)
                6,  // ORR (1S)
                9,  // STR (2N)
                36, // DMA (2N + 2(n-1)S + xI) ((N + 7S) + (N + 4 7S + 14) + 2I)
            ])
            .run(17);
    }
}
