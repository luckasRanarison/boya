pub mod registers;
pub mod types;

use crate::{
    apu::Apu,
    bus::{
        registers::{
            IORegister,
            dma::{
                DmaAddressControl, DmaData, DmaResult, DmaSpecialTiming, DmaStartTiming, DmaTimer,
            },
        },
        types::{
            Cycle, DataType, Interrupt, MemoryAccess, MemoryRegion, MemoryRegionData, WaitState,
        },
    },
    ppu::{Ppu, registers::dispstat::Dispstat},
    utils::Reset,
};

pub const BIOS_SIZE: usize = 0x04000; // 16kb
pub const IWRAM_SIZE: usize = 0x08000; // 32kb
pub const EWRAM_SIZE: usize = 0x40000; // 256kb
pub const SRAM_SIZE: usize = 0x10000; // 64kb
pub const IOREG_SIZE: usize = 0x210;

#[derive(Debug)]
pub struct GbaBus {
    pub bios: [u8; BIOS_SIZE],
    pub iwram: [u8; IWRAM_SIZE],
    pub ewram: Box<[u8; EWRAM_SIZE]>,
    pub rom: Vec<u8>,
    pub sram: Box<[u8; SRAM_SIZE]>,
    pub io: IORegister,
    pub ppu: Ppu,
    pub apu: Apu,
}

impl Default for GbaBus {
    fn default() -> Self {
        Self {
            bios: [0; BIOS_SIZE],
            iwram: [0; IWRAM_SIZE],
            ewram: Box::new([0; EWRAM_SIZE]),
            rom: Vec::new(),
            sram: Box::new([0; SRAM_SIZE]),
            io: IORegister::new(),
            ppu: Ppu::default(),
            apu: Apu::default(),
        }
    }
}

impl GbaBus {
    pub fn tick(&mut self, cycles: u32) {
        self.ppu.tick(cycles);

        let ovf0 = self.io.timer[0].tick(cycles, false);
        let ovf1 = self.io.timer[1].tick(cycles, ovf0);
        let ovf2 = self.io.timer[2].tick(cycles, ovf1);
        let _ovf3 = self.io.timer[3].tick(cycles, ovf2);

        if ovf0 {
            self.apu.on_timer_overflow(DmaTimer::Timer0);
        }

        if ovf1 {
            self.apu.on_timer_overflow(DmaTimer::Timer1);
        }

        let interrupt = self
            .ppu
            .poll_interrupt()
            .or_else(|| self.io.poll_interrupt());

        if let Some(interrupt) = interrupt {
            self.send_interrupt(interrupt);
        }
    }

    pub fn rw_cycle(&self, addr: u32, dt: DataType, access_kind: MemoryAccess) -> Cycle {
        let region = MemoryRegion::from_address(addr);
        let data = self.region_data(region);
        let access = u8::max(dt.size() / data.width.size(), 1) as u32;

        match access_kind {
            MemoryAccess::Seq => Cycle::new(0, access, 0, data.waitstate),
            MemoryAccess::NonSeq => Cycle::new(0, access - 1, 1, data.waitstate),
        }
    }

    // FIXME: currently slow, use slice copy to improve performance
    pub fn try_dma(&mut self) -> Option<DmaResult> {
        let data = self.poll_active_dma()?;
        let cycles = self.dma_cycles(&data);

        self.execute_dma(&data);

        if data.irq_enable {
            self.send_interrupt(data.channel.into());
        }

        Some(DmaResult { data, cycles })
    }

    #[inline(always)]
    fn read_rom(&self, address: usize) -> u8 {
        self.rom.get(address).copied().unwrap_or_default()
    }

    fn region_data(&self, region: MemoryRegion) -> MemoryRegionData {
        let (width, waitstate) = match region {
            MemoryRegion::BIOS => (DataType::Word, WaitState::default()),
            MemoryRegion::EWRAM => (DataType::HWord, WaitState::new(2, 2)),
            MemoryRegion::IWRAM => (DataType::Word, WaitState::default()),
            MemoryRegion::IO => (DataType::Word, WaitState::default()),
            MemoryRegion::Palette => (DataType::HWord, self.rendering_wait_state()),
            MemoryRegion::VRAM => (DataType::HWord, self.rendering_wait_state()),
            MemoryRegion::OAM => (DataType::Word, self.rendering_wait_state()),
            MemoryRegion::WaitState0 => (DataType::HWord, self.io.waitcnt.wait_state0()),
            MemoryRegion::WaitState1 => (DataType::HWord, self.io.waitcnt.wait_state1()),
            MemoryRegion::WaitState2 => (DataType::HWord, self.io.waitcnt.wait_state2()),
            MemoryRegion::SRAM => (DataType::HWord, self.io.waitcnt.sram_wait()), // FIXME: Detect save type SRAM/FLASH/EEPROM
            _ => (DataType::Word, WaitState::default()),
        };

        MemoryRegionData { width, waitstate }
    }

    fn execute_dma(&mut self, dma: &DmaData) {
        let mut src_addr = dma.src_addr;
        let mut dst_addr = dma.dst_addr;

        let dma_dt = dma.transfer_type;
        let chunk_size = dma_dt.size() as u32;

        for _ in 0..dma.transfer_len {
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

            match dma.dst_addr_ctrl {
                DmaAddressControl::Increment => dst_addr += chunk_size,
                DmaAddressControl::Decrement => dst_addr -= chunk_size,
                _ => {}
            }

            match dma.src_addr_ctrl {
                DmaAddressControl::Decrement => src_addr -= chunk_size,
                DmaAddressControl::Increment | DmaAddressControl::IncrementReload => {
                    src_addr += chunk_size
                }
                DmaAddressControl::Fixed => {}
            }
        }
    }

    fn dma_cycles(&self, dma: &DmaData) -> Cycle {
        let dma_dt = dma.transfer_type;
        let src_region = MemoryRegion::from_address(dma.src_addr);
        let dst_region = MemoryRegion::from_address(dma.dst_addr);

        let read_seq = self.rw_cycle(dma.src_addr, dma_dt, MemoryAccess::Seq);
        let write_seq = self.rw_cycle(dma.dst_addr, dma_dt, MemoryAccess::Seq);
        let read_non_seq = self.rw_cycle(dma.src_addr, dma_dt, MemoryAccess::NonSeq);
        let write_non_seq = self.rw_cycle(dma.dst_addr, dma_dt, MemoryAccess::NonSeq);

        let read_cycles = read_non_seq + read_seq.repeat(dma.transfer_len - 1);
        let write_cycles = write_non_seq + write_seq.repeat(dma.transfer_len - 1);

        let internal_cycles = match true {
            _ if src_region.is_gamepak() && dst_region.is_gamepak() => Cycle::internal(4),
            _ => Cycle::internal(2),
        };

        read_cycles + write_cycles + internal_cycles // 2N + 2(n-1)S + xI
    }

    fn poll_active_dma(&mut self) -> Option<DmaData> {
        for channel in 0..self.io.dma.len() {
            if self.should_start_dma(channel) {
                let dma = &mut self.io.dma[channel];

                if !dma.repeat() {
                    dma.disable();
                }

                return Some(dma.get_data());
            }
        }

        None
    }

    fn should_start_dma(&mut self, channel: usize) -> bool {
        let dma = &self.io.dma[channel];

        if !dma.dma_enable() {
            return false;
        }

        match dma.start_timing() {
            DmaStartTiming::Immediate => true, // FIXME: should wait 2 cycles?
            DmaStartTiming::VBlank => self.ppu.registers.dispstat.has(Dispstat::VBLANK),
            DmaStartTiming::HBlank => self.ppu.registers.dispstat.has(Dispstat::HBLANK),

            DmaStartTiming::Special => match dma.special_timing() {
                DmaSpecialTiming::None => true, // immediate
                DmaSpecialTiming::FifoA => self.apu.poll_fifo_a_request(),
                DmaSpecialTiming::FifoB => self.apu.poll_fifo_b_request(),
                DmaSpecialTiming::VideoCapture => todo!(), // TODO: Video capture DMA start
            },
        }
    }

    fn rendering_wait_state(&self) -> WaitState {
        let n = if self.ppu.is_rendering() { 1 } else { 0 };
        let s = if self.ppu.is_rendering() { 1 } else { 0 };

        WaitState::new(n, s)
    }

    fn send_interrupt(&mut self, interrupt: Interrupt) {
        if self.io.irq_master_enable() && self.io.is_irq_enabled(interrupt) {
            match interrupt {
                Interrupt::VBlank => self.ppu.mask_vblank = true,
                Interrupt::HBlank => self.ppu.mask_hblank = true,
                _ => {}
            }

            self.io.set_irq(interrupt);
        }
    }
}

impl Bus for GbaBus {
    fn read_byte(&self, address: u32) -> u8 {
        match address {
            0x0000_0000..=0x0000_3FFF => self.bios[address as usize],
            0x0200_0000..=0x02FF_FFFF => self.ewram[address as usize & 0x3FFFF],
            0x0300_0000..=0x03FF_FFFF => self.iwram[address as usize & 0x7FFF],
            0x0400_0000..=0x0400_005F => self.ppu.registers.read_byte(address),
            0x0400_0060..=0x0400_00AF => self.apu.registers.read_byte(address),
            0x0400_00B0..=0x04FF_FFFF => self.io.read_byte(address),
            0x0500_0000..=0x05FF_FFFF => self.ppu.palette[address as usize & 0x3FF],
            0x0600_0000..=0x06FF_FFFF => self.ppu.read_vram(address),
            0x0700_0000..=0x07FF_FFFF => self.ppu.oam[address as usize & 0x3FF],
            0x0800_0000..=0x0DFF_FFFF => self.read_rom(address as usize & 0x01FF_FFFF),
            0x0E00_0000..=0x0FFF_FFFF => self.sram[address as usize & 0x0FFFF],
            _ => 0x0, // TODO: open bus
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            0x0200_0000..=0x02FF_FFFF => self.ewram[address as usize & 0x3FFFF] = value,
            0x0300_0000..=0x03FF_FFFF => self.iwram[address as usize & 0x7FFF] = value,
            0x0400_0000..=0x0400_005F => self.ppu.registers.write_byte(address, value),
            0x0400_0060..=0x0400_00AF => self.apu.registers.write_byte(address, value),
            0x0400_00B0..=0x04FF_FFFF => self.io.write_byte(address, value),
            0x0500_0000..=0x05FF_FFFF => self.ppu.palette[address as usize & 0x3FF] = value,
            0x0600_0000..=0x06FF_FFFF => self.ppu.write_vram(address, value),
            0x0700_0000..=0x07FF_FFFF => self.ppu.oam[address as usize & 0x3FF] = value,
            0x0E00_0000..=0x0FFF_FFFF => self.sram[address as usize & 0x0FFFF] = value,
            _ => {}
        };
    }
}

impl Reset for GbaBus {
    fn reset(&mut self) {
        self.iwram.fill(0);
        self.ewram.fill(0);
        self.sram.fill(0);
        self.io = IORegister::new();
        self.ppu.reset();
        self.apu.reset()
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

impl<const N: usize> Bus for [u8; N] {
    fn read_byte(&self, address: u32) -> u8 {
        self[address as usize]
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        self[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::test::GbaTestBuilder;

    #[test]
    fn test_bios_cycle_count() {
        // vectors:
        //     B       reset_handler
        // reset_handler:
        //     MOV     SP, 0x0300_0000
        //     ADD     SP, SP, 0x0000_7F00
        //     MOV     PC, 0x0800_0000

        // Because Gamepak has 16bit bus width, S is divided into 2 accesses, so it becomes 4(S + waitstate) + 1N
        GbaTestBuilder::new()
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
