use crate::{
    bus::{Bus, GbaBus, registers::dma::Dma},
    ppu::registers::{bgofs::Bgofs, bgtrans::Bgtrans},
};

pub struct IOPeeker<'a>(&'a GbaBus);

impl Bus for IOPeeker<'_> {
    fn read_byte(&self, address: u32) -> u8 {
        let io = &self.0.io;
        let ppu = &self.0.ppu.registers;

        match address {
            0x0400_00B0..=0x0400_00BB => io.dma[0].peek_byte(address),
            0x0400_00BC..=0x0400_00C7 => io.dma[1].peek_byte(address),
            0x0400_00C8..=0x0400_00D3 => io.dma[2].peek_byte(address),
            0x0400_00D4..=0x0400_00DF => io.dma[3].peek_byte(address),
            0x0400_0010..=0x0400_0013 => ppu.bgofs[0].peek_byte(address),
            0x0400_0014..=0x0400_0017 => ppu.bgofs[1].peek_byte(address),
            0x0400_0018..=0x0400_001B => ppu.bgofs[2].peek_byte(address),
            0x0400_001C..=0x0400_001F => ppu.bgofs[3].peek_byte(address),
            0x0400_0020..=0x0400_002F => ppu.bg2trans.peek_byte(address),
            0x0400_0030..=0x0400_003F => ppu.bg3trans.peek_byte(address),
            _ => self.0.read_byte(address),
        }
    }

    fn write_byte(&mut self, _address: u32, _value: u8) {}
}

impl GbaBus {
    pub fn peek_hword(&self, address: u32) -> u16 {
        IOPeeker(self).read_hword(address)
    }

    pub fn peek_word(&self, address: u32) -> u32 {
        IOPeeker(self).read_word(address)
    }
}

impl Dma {
    fn peek_byte(&self, address: u32) -> u8 {
        match address % 12 {
            0..=3 => self.sad.read_byte(address),
            4..=7 => self.dad.read_byte(address),
            8..=9 => self.cnt_l.read_byte(address),
            _ => self.cnt_h.read_byte(address),
        }
    }
}

impl Bgofs {
    fn peek_byte(&self, address: u32) -> u8 {
        match address % 4 {
            0..1 => self.x.read_byte(address),
            _ => self.y.read_byte(address),
        }
    }
}

impl Bgtrans {
    fn peek_byte(&self, address: u32) -> u8 {
        match address % 16 {
            0..=1 => self.pa.read_byte(address),
            2..=3 => self.pb.read_byte(address),
            4..=5 => self.pc.read_byte(address),
            6..=7 => self.pd.read_byte(address),
            8..=11 => self.x.read_byte(address),
            _ => self.y.read_byte(address),
        }
    }
}
