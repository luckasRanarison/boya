use crate::{bus::types::WaitState, utils::bitflags::Bitflag};

#[derive(Debug, Default)]
pub struct WaitCnt {
    pub value: u16,
}

impl WaitCnt {
    pub fn sram_wait(&self) -> WaitState {
        let wait = self.two_bits_wait(0, 1);

        WaitState { n: wait, s: wait }
    }

    pub fn wait_state0(&self) -> WaitState {
        WaitState {
            n: self.two_bits_wait(2, 3),
            s: self.one_bit_wait(4, 2, 1),
        }
    }

    pub fn wait_state1(&self) -> WaitState {
        WaitState {
            n: self.two_bits_wait(5, 6),
            s: self.one_bit_wait(7, 4, 1),
        }
    }

    pub fn wait_state2(&self) -> WaitState {
        WaitState {
            n: self.two_bits_wait(8, 9),
            s: self.one_bit_wait(10, 8, 1),
        }
    }

    pub fn prefetch_buffer(&self) -> bool {
        self.value.has(14)
    }

    pub fn set_gamepak_type(&mut self, value: GamepakType) {
        match value {
            GamepakType::GBA => self.value.clear(15),
            GamepakType::GBC => self.value.set(1),
        }
    }

    #[inline(always)]
    fn one_bit_wait(&self, bit: u16, off: u8, on: u8) -> u8 {
        match self.value.get(bit) {
            0 => off,
            _ => on,
        }
    }

    #[inline(always)]
    fn two_bits_wait(&self, start: u16, end: u16) -> u8 {
        match self.value.get_bits(start, end) {
            0 => 4,
            1 => 3,
            2 => 2,
            _ => 8,
        }
    }
}

#[derive(Debug)]
pub enum GamepakType {
    GBA,
    GBC,
}
