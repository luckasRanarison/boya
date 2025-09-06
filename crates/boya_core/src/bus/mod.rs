mod register;

use crate::bus::register::SystemControl;

pub const BIOS_SIZE: usize = 0x04000;
pub const IWRAM_SIZE: usize = 0x08000;
pub const EWRAM_SIZE: usize = 0x40000;
pub const SRAM_SIZE: usize = 0x10000;

pub struct GbaBus {
    bios: [u8; BIOS_SIZE],
    iwram: [u8; IWRAM_SIZE],
    ewram: Box<[u8; EWRAM_SIZE]>,
    rom: Vec<u8>,
    sram: [u8; SRAM_SIZE],
    sys_ctrl: SystemControl,
}

impl Default for GbaBus {
    fn default() -> Self {
        Self {
            bios: [0; BIOS_SIZE],
            iwram: [0; IWRAM_SIZE],
            ewram: Box::new([0; EWRAM_SIZE]),
            rom: Vec::new(),
            sram: [0; SRAM_SIZE],
            sys_ctrl: SystemControl::default(),
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

    fn read_rom(&self, address: usize) -> u8 {
        self.rom
            .get(address - 0x0800_0000)
            .cloned()
            .unwrap_or_default()
    }

    fn read_io_register(&self, address: usize) -> u8 {
        match address {
            0x0400_0200..=0x0400_0209 => self.sys_ctrl.read(address),
            _ => panic!("invalid I/O register read address: {address:#08X}"),
        }
    }

    fn write_io_register(&mut self, address: usize, value: u8) {
        match address {
            0x0400_0200..=0x0400_0209 => self.sys_ctrl.write(address, value),
            _ => panic!("invalid I/O register write address: {address:#08X}"),
        }
    }
}

impl Bus for GbaBus {
    fn read_byte(&self, address: u32) -> u8 {
        let address = address as usize;

        match address {
            0x0000_0000..=0x0000_3FFF => self.bios[address],
            0x0200_0000..=0x0203_FFFF => self.ewram[address - 0x0200_0000],
            0x0300_0000..=0x0300_7FFF => self.iwram[address - 0x0300_0000],
            0x0400_0000..=0x0400_03EE => self.read_io_register(address),
            0x0800_0000..=0x0DFF_FFFF => self.read_rom(address),
            0x0E00_0000..=0x0E00_FFFF => self.sram[address - 0x0E00_0000],
            _ => panic!("invalid read address: {address:#08X}"),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let address = address as usize;

        match address {
            0x0200_0000..=0x0203_FFFF => self.ewram[address - 0x0200_0000] = value,
            0x0300_0000..=0x0300_7FFF => self.iwram[address - 0x0300_0000] = value,
            0x0400_0000..=0x0400_03EE => self.write_io_register(address, value),
            0x0E00_0000..=0x0E00_FFFF => self.sram[address - 0x0E00_0000] = value,
            _ => panic!("invalid write address: {address:#08X}"),
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
