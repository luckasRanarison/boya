use crate::cpu::isa::prelude::*;

/// Move shifted register
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |    Op   |         Offset5        |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    op: Opcode,
    of: u8,
    rs: u8,
    rd: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(11, 12).into();
        let of = value.get_bits_u8(6, 10);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, of, rs, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    LSL,
    LSR,
    ASR,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::LSL,
            0b01 => Self::LSR,
            0b10 => Self::ASR,
            value => unreachable!("invalid thumb 1 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let nn = self.of.imm();

        match self.op {
            Opcode::LSL => cpu.lsl(self.rd, self.rs, nn),
            Opcode::LSR => cpu.lsr(self.rd, self.rs, nn),
            Opcode::ASR => cpu.asr(self.rd, self.rs, nn),
        }
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.rd.reg().into(),
                self.rs.reg().into(),
                self.of.imm().into(),
            ],
            kind: InstructionKind::thumb(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_shift() {
        let asm = r"
            mov r1, #2
            lsl r2, r1, #2
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 8)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(2);
    }

    #[test]
    fn test_arithmetic_shift() {
        let asm = r"
            mov r0, #0
            mvn r1, r0
            asr r2, r1, #1
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, !0)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .run(3);
    }

    #[test]
    fn test_special_lsr() {
        let asm = r"
            lsr r1, r4, #0x10
            mov r2, #0x2
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 2)
            .run(2);
    }
}
