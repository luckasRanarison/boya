use crate::cpu::isa::prelude::*;

/// Single data swap
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0 1 0|B|0|0|  Rn   |  Rd   |0 0 0 0 1 0 0 1|  Rm    |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    b: bool,
    rd: u8,
    rn: u8,
    rm: u8,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SWP{:?}{} {:?}, {:?}, [{:?}]",
            self.cd,
            if self.b { "B" } else { "" },
            self.rd.reg(),
            self.rm.reg(),
            self.rn.reg()
        )
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let b = value.has(22);
        let rn = value.get_bits_u8(16, 19);
        let rd = value.get_bits_u8(12, 15);
        let rm = value.get_bits_u8(0, 3);

        Self { cd, b, rd, rn, rm }
    }
}

impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        cpu.swp(self.rd, self.rm, self.rn, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let asm = r"
            MOV    R0, #5
            MOV    R1, #25
            MOV    R2, 0x0200_0000
            SWPB   R0, R1, [R2]
            ADD    R5, R2, 0x20
            SWP    R3, R0, [R5]
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.write_byte(0x0200_0000, 40);
                cpu.bus.write_word(0x0200_0020, 0xFFFF0000);
            })
            .assert_reg(0, 40)
            .assert_reg(3, 0xFFFF0000)
            .assert_byte(0x0200_0000, 25)
            .assert_word(0x0200_0020, 40)
            .run(6);
    }
}
