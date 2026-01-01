pub mod types;

use crate::{
    bus::types::{
        Cycle, DataType, Interrupt, MemoryAccess, MemoryRegion, MemoryRegionData, WaitState,
    },
    ppu::Ppu,
    registers::{
        io::{
            IORegister,
            dma::{Dma, DmaAddressControl, DmaSpecialTiming, DmaStartTiming},
        },
        ppu::dispstat::Dispstat,
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
        let registers = &mut self.registers;

        self.ppu.tick(cycles);

        let timer0_ovf = registers.timer0.tick(cycles, false);
        let timer1_ovf = registers.timer1.tick(cycles, timer0_ovf);
        let timer2_ovf = registers.timer2.tick(cycles, timer1_ovf);
        let _ = registers.timer3.tick(cycles, timer2_ovf);

        let interrupt = self
            .ppu
            .poll_interrupt()
            .or_else(|| registers.timer0.poll_interrupt())
            .or_else(|| registers.timer1.poll_interrupt())
            .or_else(|| registers.timer2.poll_interrupt())
            .or_else(|| registers.timer3.poll_interrupt())
            .or_else(|| registers.keypad.poll_interrupt());

        if let Some(interrupt) = interrupt {
            self.set_interrupt(interrupt);
        }
    }

    pub fn load_bios(&mut self, bios: &[u8; BIOS_SIZE]) {
        self.bios = *bios;
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
    }

    pub fn poll_interrupt(&self) -> bool {
        self.registers.has_pending_irq()
    }

    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        if self.registers.ime.has(0) && self.registers.ie.has(interrupt as u16) {
            self.registers.irf.set(interrupt as u16);
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
            MemoryRegion::Palette => (DataType::HWord, WaitState::default()), // >
            MemoryRegion::VRAM => (DataType::HWord, WaitState::default()),    // >
            MemoryRegion::OAM => (DataType::Word, WaitState::default()), //      > FIXME: +1 during rendering
            MemoryRegion::WaitState0 => (DataType::HWord, self.registers.waitcnt.wait_state0()),
            MemoryRegion::WaitState1 => (DataType::HWord, self.registers.waitcnt.wait_state1()),
            MemoryRegion::WaitState2 => (DataType::HWord, self.registers.waitcnt.wait_state2()),
            MemoryRegion::SRAM => (DataType::HWord, self.registers.waitcnt.sram_wait()), // FIXME: Detect save type SRAM/FLASH/EEPROM
            _ => (DataType::Word, WaitState::default()),
        };

        MemoryRegionData { width, waitstate }
    }

    // FIXME: currently slow, use slice copy to improve performance
    pub fn try_dma(&mut self) -> Option<Cycle> {
        let dma = self.get_active_dma()?;

        if !dma.repeat() {
            dma.disable();
        }

        let dma_image = dma.clone();
        let cycles = self.get_dma_cycles(&dma_image);

        self.execute_dma(&dma_image);

        if dma_image.irq_enable() {
            self.set_interrupt(dma_image.channel.into());
        }

        Some(cycles)
    }

    fn execute_dma(&mut self, dma: &Dma) {
        let mut src_addr = dma.sad;
        let mut dst_addr = dma.dad;

        let dma_dt = dma.transfer_type();
        let chunk_size = dma_dt.size() as u32;

        for _ in 0..dma.transfer_len() {
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

            match dma.dst_addr_control() {
                DmaAddressControl::Increment => dst_addr += chunk_size,
                DmaAddressControl::Decrement => dst_addr -= chunk_size,
                _ => {}
            }

            match dma.src_addr_control() {
                DmaAddressControl::Decrement => src_addr -= chunk_size,
                DmaAddressControl::Increment | DmaAddressControl::IncrementReload => {
                    src_addr += chunk_size
                }
                DmaAddressControl::Fixed => {}
            }
        }
    }

    fn get_dma_cycles(&self, dma: &Dma) -> Cycle {
        let dma_dt = dma.transfer_type();
        let src_region = MemoryRegion::from_address(dma.sad);
        let dst_region = MemoryRegion::from_address(dma.dad);

        let read_cycles_seq = self.get_rw_cycle(dma.sad, dma_dt, MemoryAccess::Seq);
        let write_cycles_seq = self.get_rw_cycle(dma.dad, dma_dt, MemoryAccess::Seq);
        let read_cycles_non_seq = self.get_rw_cycle(dma.sad, dma_dt, MemoryAccess::NonSeq);
        let write_cycles_non_seq = self.get_rw_cycle(dma.dad, dma_dt, MemoryAccess::NonSeq);

        let read_cycles = read_cycles_non_seq + read_cycles_seq.repeat(dma.transfer_len() - 1);
        let write_cycles = write_cycles_non_seq + write_cycles_seq.repeat(dma.transfer_len() - 1);

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
            DmaStartTiming::VBlank => self.ppu.registers.dispstat.has(Dispstat::VBLANK),
            DmaStartTiming::HBlank => self.ppu.registers.dispstat.has(Dispstat::HBLANK),

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
}
