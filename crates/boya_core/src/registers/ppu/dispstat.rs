use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct Dispstat {
    pub value: u16,
}

impl Dispstat {
    pub fn vblank(&self) -> bool {
        self.value.has(0)
    }

    pub fn hblank(&self) -> bool {
        self.value.has(1)
    }

    pub fn vcount(&self) -> bool {
        self.value.has(2)
    }

    pub fn vblank_irq_enable(&self) -> bool {
        self.value.has(3)
    }

    pub fn hblank_irq_enable(&self) -> bool {
        self.value.has(4)
    }

    pub fn vcounter_irq_enable(&self) -> bool {
        self.value.has(5)
    }

    pub fn get_vcount(&self) -> u16 {
        self.value.get_bits(8, 15)
    }
}
