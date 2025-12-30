pub mod registers;
pub mod types;

use crate::{
    bus::{
        registers::{IORegister, interrupt::Interrupt},
        types::{DataType, MemoryRegion, WaitState},
    },
    ppu::Ppu,
    utils::bitflags::Bitflag,
};

pub const BIOS_SIZE: usize = 0x04000; // 16kb
pub const IWRAM_SIZE: usize = 0x08000; // 32kb
pub const EWRAM_SIZE: usize = 0x40000; // 256kb
pub const SRAM_SIZE: usize = 0x10000; // 64kb

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
            registers: IORegister::new(),
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

    pub fn poll_interrupt(&self) -> bool {
        self.registers.irf.value != 0
    }

    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        if self.registers.ime != 0 && self.registers.ie.has(interrupt as u16) {
            self.registers.irf.value.set(interrupt as u16);
        }
    }

    pub fn get_region_data(&self, address: u32) -> MemoryRegion {
        let (width, waitstate) = match address {
            0x0000_0000..=0x0000_3FFF => (DataType::Word, WaitState::default()), // BIOS
            0x0200_0000..=0x0203_FFFF => (DataType::HWord, WaitState { n: 2, s: 2 }), // EWRAM
            0x0300_0000..=0x0300_7FFF => (DataType::Word, WaitState::default()), // IWRAM
            0x0400_0000..=0x0400_03FE => (DataType::Word, WaitState::default()), // I/O
            0x0500_0000..=0x0500_03FF => (DataType::HWord, WaitState::default()), // PALETTE >
            0x0600_0000..=0x0617_FFFF => (DataType::HWord, WaitState::default()), // VRAM    >
            0x0700_0000..=0x0700_03FF => (DataType::Word, WaitState::default()), //  OAM     > FIXME: +1 during rendering
            0x0800_0000..=0x09FF_FFFF => (DataType::HWord, self.registers.waitcnt.wait_state0()),
            0x0A00_0000..=0x0BFF_FFFF => (DataType::HWord, self.registers.waitcnt.wait_state1()),
            0x0C00_0000..=0x0DFF_FFFF => (DataType::HWord, self.registers.waitcnt.wait_state2()),
            0x0E00_0000..=0x0E00_FFFF => (DataType::HWord, self.registers.waitcnt.sram_wait()), // FIXME: Detect save type SRAM/FLASH/EEPROM
            _ => (DataType::Word, WaitState::default()), // out of bounds!
        };

        MemoryRegion { width, waitstate }
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

#[cfg(test)]
mod tests {
    use crate::test::AsmTestBuilder;

    #[test]
    fn test_bios_cycle_count() {
        // vectors:
        //     B       reset_handler        ; 2S + 1N (3)
        // reset_handler:
        //     MOV     SP, 0x0300_0000      ; 1S (1)
        //     ADD     SP, SP, 0x0000_7F00  ; 1S (1)
        //     MOV     PC, 0x0800_0000      ; 2S + 1N (13)

        // NOTE: Because Gamepak has 16bit bus width, S is divided into 2 accesses, so it becomes 4(S + waitstate) + 1N

        AsmTestBuilder::new()
            .pc(0x00)
            .assert_cycles([3, 1, 1, 13])
            .run(4);
    }

    #[test]
    fn test_waitstate() {
        let asm = r"
            ; set waitstate 0 to 3,1
            MOV     R0, #10100b      ; 1S (6)
            MOV     R1, #0x0400_0000 ; 1S (6)
            MOV     R2, #0x0000_0200 ; 1S (6)
            ADD     R3, R1, R2       ; 1S (6)
            STR     R0, [R3, #4]     ; 2N (9) (N + 4 + S + 2) + N
            MOV     R4, R0           ; 1S (4)
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_cycles([6, 6, 6, 6, 9, 4])
            .run(6);
    }
}
