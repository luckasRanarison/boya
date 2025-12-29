use crate::{bus::Bus, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct Dma {
    pub sad: u32,
    pub dad: u32,
    pub cnt_l: u16,
    pub cnt_h: u16,
}

impl Dma {
    pub fn dst_addr_control(&self) -> DmaAddressControl {
        match self.cnt_h.get_bits(5, 6) {
            0 => DmaAddressControl::Increment,
            1 => DmaAddressControl::Decrement,
            2 => DmaAddressControl::Fixed,
            _ => DmaAddressControl::IncrementReload,
        }
    }

    pub fn src_addr_control(&self) -> DmaAddressControl {
        match self.cnt_h.get_bits(5, 6) {
            0 => DmaAddressControl::Increment,
            1 => DmaAddressControl::Decrement,
            2 => DmaAddressControl::Fixed,
            _ => unreachable!("invalid address control, prohibited"),
        }
    }

    pub fn dma_repeat(&self) -> bool {
        self.cnt_h.has(7)
    }

    pub fn transfer_type(&self) -> DmaTransferType {
        match self.cnt_h.get(10) {
            0 => DmaTransferType::Dma16,
            _ => DmaTransferType::Dma32,
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
}

impl Bus for Dma {
    fn read_byte(&self, address: u32) -> u8 {
        match address % 12 {
            10..=11 => self.cnt_h.read_byte(address),
            _ => unreachable!("invalid DMA register read"),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address % 12 {
            0..=3 => self.sad.write_byte(address, value),
            4..=7 => self.dad.write_byte(address, value),
            8..=10 => self.cnt_l.write_byte(address, value),
            _ => self.cnt_h.write_byte(address, value),
        }
    }
}

#[derive(Debug)]
pub enum DmaAddressControl {
    Increment,
    Decrement,
    Fixed,
    IncrementReload,
}

#[derive(Debug)]
pub enum DmaTransferType {
    Dma16,
    Dma32,
}

#[derive(Debug)]
pub enum DmaStartTiming {
    Immediate,
    VBlank,
    HBlank,
    Special,
}
