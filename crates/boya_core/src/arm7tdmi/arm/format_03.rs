use crate::arm7tdmi::isa::prelude::*;

/// Branch X
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0 1 0 0 1 0 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 1|   Rn   |
/// +-----------------------------------------------------------------+
pub struct Format3 {
    cd: Condition,
    rn: u8,
}

impl Debug for Format3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BX{:?} {:?}", self.cd, self.rn.reg())
    }
}

impl From<u32> for Format3 {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let rn = value.get_bits_u8(0, 3);

        Self { cd, rn }
    }
}
impl<B: Bus> Executable<B> for Format3 {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        cpu.bx_op(self.rn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bx() {
        let asm = r"
            MOV    R0, #1011b  
            BX     R0 ; + pre-fetch 4
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(15, 14)
            .assert_flag(Psr::T, true)
            .run(2);
    }
}
