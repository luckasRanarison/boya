use crate::arm7tdmi::isa::prelude::*;

/// Branch and Branch with Link
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |1 0 1|L|                      Offset                    |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    nn: i32,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?} {:?}", self.op, self.cd, self.nn)
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let op = value.get_u8(24).into();
        let nn = ((value.get_bits(0, 23) << 9) as i32) >> 7; // sign-exteneded + steps 4

        Self { cd, op, nn }
    }
}

#[derive(Debug)]
enum Opcode {
    B,
    BL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::B,
            0x1 => Self::BL,
            _ => unreachable!("invalid arm 4 opcode: {value:#b}"),
        }
    }
}
impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        if matches!(self.op, Opcode::BL) {
            let lr = NamedRegister::LR as usize;
            let pc = cpu.next_instr_addr().unwrap();

            cpu.set_reg(lr, pc);
        }

        cpu.b(self.nn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branching_link() {
        let asm = r"
            main:
                BL      target ; 0
                MOV     R0, #1 ; 4

            back:
                MOV     R1, #3 ; 8

            target:
                MOV     R0, #2 ; 12
                B       back   ; 16
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(0, 2)
            .assert_reg(1, 3)
            .assert_reg(14, ARM_MAIN_START + 4)
            .assert_reg(15, ARM_MAIN_START + 20) // + pre-fetch 8
            .run(4);
    }
}
