use crate::cpu::isa::prelude::*;

/// Software interrupt
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  1 |  1 |  1 |  1 |  1 |                 Value8                |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    nn: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let nn = value.get_bits_u8(0, 7);

        Self { nn }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        cpu.swi()
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: "SWI".into(),
            args: vec![self.nn.imm().into()],
            kind: InstructionKind::thumb(17),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swi() {
        let asm = r"
            main:
                swi  #72    ; 0

            dead:
                mov  r0, #1 ; 2
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_flag(Psr::T, false)
            .assert_reg(0, 0)
            .assert_reg(15, 16)
            .run(1)
    }
}
