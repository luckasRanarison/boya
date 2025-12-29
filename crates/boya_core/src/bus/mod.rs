pub mod registers;

use crate::{bus::registers::IORegister, ppu::Ppu};

pub const BIOS_SIZE: usize = 0x04000; // 16kb
pub const IWRAM_SIZE: usize = 0x08000; // 32kb
pub const EWRAM_SIZE: usize = 0x40000; // 256kb
pub const SRAM_SIZE: usize = 0x10000; // 64kb

#[derive(Debug)]
pub enum DataType {
    Byte,
    HWord,
    Word,
}

#[derive(Debug, Default)]
pub struct WaitState {
    pub n: u8,
    pub s: u8,
}

#[derive(Debug)]
pub struct GbaBus {
    bios: [u8; BIOS_SIZE],
    iwram: [u8; IWRAM_SIZE],
    ewram: Box<[u8; EWRAM_SIZE]>,
    rom: Vec<u8>,
    sram: Box<[u8; SRAM_SIZE]>,
    registers: IORegister,
    ppu: Ppu,
}

impl Default for GbaBus {
    fn default() -> Self {
        Self {
            bios: [0; BIOS_SIZE],
            iwram: [0; IWRAM_SIZE],
            ewram: Box::new([0; EWRAM_SIZE]),
            rom: Vec::new(),
            sram: Box::new([0; SRAM_SIZE]),
            registers: IORegister::default(),
            ppu: Ppu::default(),
        }
    }
}

impl GbaBus {
    pub fn load_bios(&mut self, bios: &[u8; BIOS_SIZE]) {
        self.bios = *bios;
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
    }

    pub fn get_waitstate(&self, address: u32) -> WaitState {
        match address {
            0x0000_0000..=0x0000_3FFF => WaitState::default(), // BIOS
            0x0200_0000..=0x0203_FFFF => WaitState { n: 2, s: 2 }, // EWRAM
            0x0300_0000..=0x0300_7FFF => WaitState::default(), // IWRAM
            0x0400_0000..=0x0400_03FE => WaitState::default(), // I/O
            0x0500_0000..=0x0500_03FF => WaitState::default(), // PALETTE >
            0x0600_0000..=0x0617_FFFF => WaitState::default(), // VRAM    >
            0x0700_0000..=0x0700_03FF => WaitState::default(), // OAM     > FIXME: +1 during rendering
            0x0800_0000..=0x09FF_FFFF => self.registers.waitcnt.wait_state0(),
            0x0A00_0000..=0x0BFF_FFFF => self.registers.waitcnt.wait_state1(),
            0x0C00_0000..=0x0DFF_FFFF => self.registers.waitcnt.wait_state2(),
            0x0E00_0000..=0x0E00_FFFF => self.registers.waitcnt.sram_wait(), // FIXME: Detect save type SRAM/FLASH/EEPROM
            _ => unreachable!("invalid read/write address access: {address:08X}"),
        }
    }

    fn read_rom(&self, address: u32) -> u8 {
        self.rom
            .get(address as usize - 0x0800_0000)
            .cloned()
            .unwrap_or_default()
    }
}

impl Bus for GbaBus {
    fn read_byte(&self, address: u32) -> u8 {
        match address {
            0x0000_0000..=0x0000_3FFF => self.bios[address as usize],
            0x0200_0000..=0x0203_FFFF => self.ewram[address as usize - 0x0200_0000],
            0x0300_0000..=0x0300_7FFF => self.iwram[address as usize - 0x0300_0000],
            0x0400_0000..=0x0400_005F => self.ppu.registers.read_byte(address),
            0x0400_00B0..=0x0400_03FE => self.registers.read_byte(address),
            0x0500_0000..=0x0500_03FF => self.ppu.palette[address as usize - 0x0500_0000],
            0x0600_0000..=0x0617_FFFF => self.ppu.vram[address as usize - 0x0600_0000],
            0x0700_0000..=0x0700_03FF => self.ppu.oam[address as usize - 0x0700_0000],
            0x0800_0000..=0x0DFF_FFFF => self.read_rom(address),
            0x0E00_0000..=0x0E00_FFFF => self.sram[address as usize - 0x0E00_0000],
            _ => 0x0, // open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            0x0200_0000..=0x0203_FFFF => self.ewram[address as usize - 0x0200_0000] = value,
            0x0300_0000..=0x0300_7FFF => self.iwram[address as usize - 0x0300_0000] = value,
            0x0400_0000..=0x0400_005F => self.ppu.registers.write_byte(address, value),
            0x0400_00B0..=0x0400_03FE => self.registers.write_byte(address, value),
            0x0500_0000..=0x0500_03FF => self.ppu.palette[address as usize - 0x0500_0000] = value,
            0x0600_0000..=0x0617_FFFF => self.ppu.vram[address as usize - 0x0600_0000] = value,
            0x0700_0000..=0x0700_03FF => self.ppu.oam[address as usize - 0x0700_0000] = value,
            0x0E00_0000..=0x0E00_FFFF => self.sram[address as usize - 0x0E00_0000] = value,
            _ => {}
        };
    }
}

pub trait Bus {
    fn read_byte(&self, address: u32) -> u8;
    fn write_byte(&mut self, address: u32, value: u8);

    fn read_hword(&self, address: u32) -> u16 {
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);
        u16::from_le_bytes([b1, b2])
    }

    fn write_hword(&mut self, address: u32, value: u16) {
        let [b1, b2] = value.to_le_bytes();
        self.write_byte(address, b1);
        self.write_byte(address + 1, b2);
    }

    fn read_word(&self, address: u32) -> u32 {
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);
        let b3 = self.read_byte(address + 2);
        let b4 = self.read_byte(address + 3);
        u32::from_le_bytes([b1, b2, b3, b4])
    }

    fn write_word(&mut self, address: u32, value: u32) {
        let [b1, b2, b3, b4] = value.to_le_bytes();
        self.write_byte(address, b1);
        self.write_byte(address + 1, b2);
        self.write_byte(address + 2, b3);
        self.write_byte(address + 3, b4);
    }
}

impl Bus for u16 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 2;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 2;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u16::from_le_bytes(bytes);
    }
}

impl Bus for u32 {
    fn read_byte(&self, address: u32) -> u8 {
        let index = address % 4;
        let bytes = self.to_le_bytes();

        bytes[index as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let index = address % 4;
        let mut bytes = self.to_le_bytes();

        bytes[index as usize] = value;
        *self = u32::from_le_bytes(bytes);
    }
}
