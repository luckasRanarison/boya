use crate::{
    bus::{Bus, types::DataType},
    utils::bitflags::Bitflag,
};

#[derive(Debug, Default)]
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
            (0, DmaChannel::DMA3) => 0x10000,
            (_, DmaChannel::DMA3) => self.cnt_l as u32,
            (0, _) => 0x4000,
            (_, _) => self.cnt_l as u32 & 0x3FFF,
        }
    }

    pub fn get_special_timing(&self) -> DmaSpecialTiming {
        self.channel.get_special_timing()
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
    DMA0,
    DMA1,
    DMA2,
    DMA3,
}

impl DmaChannel {
    pub fn get_special_timing(self) -> DmaSpecialTiming {
        match self {
            DmaChannel::DMA0 => DmaSpecialTiming::None,
            DmaChannel::DMA1 => DmaSpecialTiming::FifoA,
            DmaChannel::DMA2 => DmaSpecialTiming::FifoB,
            DmaChannel::DMA3 => DmaSpecialTiming::VideoCapture,
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
