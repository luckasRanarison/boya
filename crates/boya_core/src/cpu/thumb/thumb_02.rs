use crate::cpu::isa::prelude::*;

/// Add/Substract
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |  1 |  1 |  I | Op |  Rn/Offset3  |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    op: Opcode,
    nn: Operand,
    rs: u8,
    rd: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(9, 10).into();
        let operand = value.get_bits(6, 8);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        let nn = match value.has(10) {
            true => operand.imm(),
            false => operand.reg(),
        };

        Self { op, nn, rs, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    ADD,
    SUB,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0b00 | 0b10 => Self::ADD,
            0b01 | 0b11 => Self::SUB,
            value => unreachable!("invalid thumb 2 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::ADD => cpu.add(self.rd, self.rs, self.nn, true),
            Opcode::SUB => cpu.sub(self.rd, self.rs, self.nn, true),
        }
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![
                self.rd.reg().into(),
                self.rs.reg().into(),
                self.nn.clone().into(),
            ],
            kind: InstructionKind::thumb(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sub_basic() {
        let asm = r"
            mov r0, #2
            mov r1, #3
            add r2, r1, r0
            add r3, r2, #3
            sub r4, r3, r0
            cmp r4, #6
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 5)
            .assert_reg(3, 8)
            .assert_reg(4, 6)
            .assert_flag(Psr::C, true) // no borrow
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(6);
    }
}
