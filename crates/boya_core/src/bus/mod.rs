pub mod types;

use crate::{
    bus::types::{Cycle, DataType, MemoryAccess, MemoryRegion, MemoryRegionData, WaitState},
    ppu::Ppu,
    registers::io::{
        IORegister,
        dma::{Dma, DmaAddressControl, DmaSpecialTiming, DmaStartTiming},
        interrupt::Interrupt,
    },
    utils::bitflags::Bitflag,
};

pub const BIOS_SIZE: usize = 0x04000; // 16kb
pub const IWRAM_SIZE: usize = 0x08000; // 32kb
pub const EWRAM_SIZE: usize = 0x40000; // 256kb
pub const SRAM_SIZE: usize = 0x10000; // 64kb

#[derive(Debug)]
pub struct GbaBus {
    pub bios: [u8; BIOS_SIZE],
    pub iwram: [u8; IWRAM_SIZE],
    pub ewram: Box<[u8; EWRAM_SIZE]>,
    pub rom: Vec<u8>,
    pub sram: Box<[u8; SRAM_SIZE]>,
    pub registers: IORegister,
    pub ppu: Ppu,
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
    pub fn tick(&mut self, cycles: u32) {
        self.ppu.tick(cycles);
    }

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

    pub fn get_rw_cycle(&self, addr: u32, dt: DataType, access_kind: MemoryAccess) -> Cycle {
        let region = MemoryRegion::from_address(addr);
        let data = self.get_region_data(region);
        let access = u8::max(dt.size() / data.width.size(), 1) as u32;

        match access_kind {
            MemoryAccess::Seq => Cycle::new(0, access, 0, data.waitstate),
            MemoryAccess::NonSeq => Cycle::new(0, access - 1, 1, data.waitstate),
        }
    }

    pub fn get_region_data(&self, region: MemoryRegion) -> MemoryRegionData {
        let (width, waitstate) = match region {
            MemoryRegion::BIOS => (DataType::Word, WaitState::default()),
            MemoryRegion::EWRAM => (DataType::HWord, WaitState { n: 2, s: 2 }),
            MemoryRegion::IWRAM => (DataType::Word, WaitState::default()),
            MemoryRegion::IO => (DataType::Word, WaitState::default()),
            MemoryRegion::PALETTE => (DataType::HWord, WaitState::default()), // >
            MemoryRegion::VRAM => (DataType::HWord, WaitState::default()),    // >
            MemoryRegion::OAM => (DataType::Word, WaitState::default()), //      > FIXME: +1 during rendering
            MemoryRegion::WAITSTATE0 => (DataType::HWord, self.registers.waitcnt.wait_state0()),
            MemoryRegion::WAITSTATE1 => (DataType::HWord, self.registers.waitcnt.wait_state1()),
            MemoryRegion::WAITSTATE2 => (DataType::HWord, self.registers.waitcnt.wait_state2()),
            MemoryRegion::SRAM => (DataType::HWord, self.registers.waitcnt.sram_wait()), // FIXME: Detect save type SRAM/FLASH/EEPROM
            _ => (DataType::Word, WaitState::default()),
        };

        MemoryRegionData { width, waitstate }
    }

    // FIXME: currently slow, use slice copy to improve performance
    pub fn start_dma(&mut self) -> Option<Cycle> {
        let dma = self.get_active_dma()?;

        if !dma.repeat() {
            dma.disable();
        }

        let mut src_addr = dma.sad;
        let mut dst_addr = dma.dad;

        let src_addr_ctrl = dma.src_addr_control();
        let dst_addr_ctrl = dma.dst_addr_control();
        let dma_len = dma.transfer_len();
        let dma_dt = dma.transfer_type();
        let chunk_size = dma_dt.size() as u32;

        let cycles = self.compute_dma_cycles(src_addr, dst_addr, dma_len, dma_dt);

        for _ in 0..dma_len {
            match dma_dt {
                DataType::Word => {
                    let word = self.read_word(src_addr);
                    self.write_word(dst_addr, word);
                }
                _ => {
                    let hword = self.read_hword(src_addr);
                    self.write_hword(dst_addr, hword);
                }
            }

            match dst_addr_ctrl {
                DmaAddressControl::Increment => dst_addr += chunk_size,
                DmaAddressControl::Decrement => dst_addr -= chunk_size,
                _ => {}
            }

            match src_addr_ctrl {
                DmaAddressControl::Decrement => src_addr -= chunk_size,
                DmaAddressControl::Increment | DmaAddressControl::IncrementReload => {
                    src_addr += chunk_size
                }
                DmaAddressControl::Fixed => {}
            }
        }

        Some(cycles)
    }

    fn compute_dma_cycles(
        &self,
        src_addr: u32,
        dst_addr: u32,
        dma_len: u32,
        dma_dt: DataType,
    ) -> Cycle {
        let src_region = MemoryRegion::from_address(src_addr);
        let dst_region = MemoryRegion::from_address(dst_addr);

        let read_cycles_seq = self.get_rw_cycle(src_addr, dma_dt, MemoryAccess::Seq);
        let write_cycles_seq = self.get_rw_cycle(dst_addr, dma_dt, MemoryAccess::Seq);
        let read_cycles_non_seq = self.get_rw_cycle(src_addr, dma_dt, MemoryAccess::NonSeq);
        let write_cycles_non_seq = self.get_rw_cycle(dst_addr, dma_dt, MemoryAccess::NonSeq);

        let read_cycles = read_cycles_non_seq + read_cycles_seq.repeat(dma_len - 1);
        let write_cycles = write_cycles_non_seq + write_cycles_seq.repeat(dma_len - 1);

        let internal_cycles = match (src_region, dst_region) {
            _ if src_region.is_gamepak() && dst_region.is_gamepak() => Cycle::internal(4),
            _ => Cycle::internal(2),
        };

        read_cycles + write_cycles + internal_cycles // 2N + 2(n-1)S + xI
    }

    fn get_active_dma(&mut self) -> Option<&mut Dma> {
        match true {
            _ if self.should_start_dma(&self.registers.dma0) => Some(&mut self.registers.dma0),
            _ if self.should_start_dma(&self.registers.dma1) => Some(&mut self.registers.dma1),
            _ if self.should_start_dma(&self.registers.dma2) => Some(&mut self.registers.dma2),
            _ if self.should_start_dma(&self.registers.dma3) => Some(&mut self.registers.dma3),
            _ => None,
        }
    }

    fn should_start_dma(&self, dma: &Dma) -> bool {
        if !dma.dma_enable() {
            return false;
        }

        match dma.start_timing() {
            DmaStartTiming::Immediate => true, // FIXME: should wait 2 cycles?
            DmaStartTiming::VBlank => self.ppu.registers.dispstat.vblank(),
            DmaStartTiming::HBlank => self.ppu.registers.dispstat.hblank(),

            DmaStartTiming::Special => match dma.get_special_timing() {
                DmaSpecialTiming::None => true, // immediate
                DmaSpecialTiming::FifoA => todo!("FIFO_A DMA start"),
                DmaSpecialTiming::FifoB => todo!("FIFO_B DMA start"),
                DmaSpecialTiming::VideoCapture => todo!("Video capture DMA start"),
            },
        }
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
            0x0800_0000..=0x0DFF_FFFF => self.rom[address as usize - 0x0800_0000],
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
        //     B       reset_handler
        // reset_handler:
        //     MOV     SP, 0x0300_0000
        //     ADD     SP, SP, 0x0000_7F00
        //     MOV     PC, 0x0800_0000

        // Because Gamepak has 16bit bus width, S is divided into 2 accesses, so it becomes 4(S + waitstate) + 1N
        AsmTestBuilder::new()
            .pc(0x00)
            .assert_cycles([
                3,  // B   (2S + 1N)
                1,  // MOV (1S)
                1,  // ADD (1S)
                13, // MOV (2S + 1N)
            ])
            .run(4);
    }

    #[test]
    fn test_waitstate() {
        let asm = r"
            ; set waitstate 0 to 3,1
            MOV     R0, #10100b
            MOV     R1, #0x0400_0000
            MOV     R2, #0x0000_0200
            ADD     R3, R1, R2
            STRH    R0, [R3, #4]
            MOV     R4, R0
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_cycles([
                6, // MOV  (1S)
                6, // MOV  (1S)
                6, // MOV  (1S)
                6, // ADD  (1S)
                9, // STRH (2N) (N + 4 + S + 2) + N
                4, // MOV  (1S)
            ])
            .run(6);
    }

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
                ORR     R2, R2, #0x8000
                STRH    R2, [R0, R1]
        ";

        let expected_chunks = vec![0x5010, 0x10FF, 0x2050, 0xA030]
            .into_iter()
            .map(|c: u32| c.to_le_bytes())
            .flatten()
            .collect::<Vec<_>>();

        AsmTestBuilder::new()
            .asm(asm)
            .assert_fn(move |cpu| {
                let dma0 = &cpu.bus.registers.dma0;

                assert_eq!(0x0800_0004, dma0.sad, "DMA0 source address");
                assert_eq!(0x0300_0000, dma0.dad, "DMA0 destination address");
                assert_eq!(8, dma0.transfer_len(), "DMA0 transfer length");
                assert_eq!(&cpu.bus.iwram[..16], &expected_chunks);
                assert!(!dma0.dma_enable(), "DMA0 should be disabled");
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
                9,  // STR (2N)
                36, // DMA (2N + 2(n-1)S + xI) ((N + 7S) + (N + 4 7S + 14) + 2I)
            ])
            .run(16);
    }
}
