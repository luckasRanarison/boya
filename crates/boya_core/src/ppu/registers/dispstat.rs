use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct Dispstat {
    pub value: u16,
}

impl Dispstat {
    pub fn get_vcount(&self) -> u16 {
        self.value.get_bits(8, 15)
    }
}
