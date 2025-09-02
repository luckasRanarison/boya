use crate::arm7tdmi::isa::prelude::*;

/// Branch X
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0 1 0 0 1 0 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 1|   Rn   |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    rn: u8,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BX{:?} {:?}", self.cd, self.rn.reg())
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let rn = value.get_bits_u8(0, 3);

        Self { cd, rn }
    }
}
impl<B: Bus> Executable<B> for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        cpu.bx(self.rn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bx() {
        let asm = r"
            MOV     R0, 0x224
            ORR     R0, R0, #1
            BX      R0 ; + pre-fetch 4
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(15, 0x228)
            .assert_flag(Psr::T, true)
            .run(3);
    }
}
