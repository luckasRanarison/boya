use crate::utils::bitflags::Bitflag;

#[derive(Debug, Default)]
pub struct Mosaic {
    pub value: u16,
}

impl Mosaic {
    pub fn bg_mosaic_hsize(&self) -> u16 {
        self.value.get_bits(0, 3)
    }

    pub fn bg_mosaic_vsize(&self) -> u16 {
        self.value.get_bits(4, 7)
    }

    pub fn obj_mosaic_hsize(&self) -> u16 {
        self.value.get_bits(8, 11)
    }

    pub fn obj_mosaic_vsize(&self) -> u16 {
        self.value.get_bits(12, 15)
    }
}
