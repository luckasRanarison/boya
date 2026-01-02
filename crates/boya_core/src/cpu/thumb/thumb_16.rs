use crate::cpu::isa::prelude::*;

/// Conditional branch
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  1 |        Cond       |               SOffset8                |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    of: i16,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(8, 11).into();
        let of = (value.get_bits(0, 7) as i8 as i16) << 1;

        Self { op, of }
    }
}

#[derive(Debug)]
enum Opcode {
    BEQ,
    BNE,
    BCS,
    BCC,
    BMI,
    BPL,
    BVS,
    BVC,
    BHI,
    BLS,
    BGE,
    BLT,
    BGT,
    BLE,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::BEQ,
            0x1 => Self::BNE,
            0x2 => Self::BCS,
            0x3 => Self::BCC,
            0x4 => Self::BMI,
            0x5 => Self::BPL,
            0x6 => Self::BVS,
            0x7 => Self::BVC,
            0x8 => Self::BHI,
            0x9 => Self::BLS,
            0xA => Self::BGE,
            0xB => Self::BLT,
            0xC => Self::BGT,
            0xD => Self::BLE,
            _ => unreachable!("invalid thumb 16 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::BEQ => cpu.beq(self.of),
            Opcode::BNE => cpu.bne(self.of),
            Opcode::BCS => cpu.bcs(self.of),
            Opcode::BCC => cpu.bcc(self.of),
            Opcode::BMI => cpu.bmi(self.of),
            Opcode::BPL => cpu.bpl(self.of),
            Opcode::BVS => cpu.bvs(self.of),
            Opcode::BVC => cpu.bvc(self.of),
            Opcode::BHI => cpu.bhi(self.of),
            Opcode::BLS => cpu.bls(self.of),
            Opcode::BGE => cpu.bge(self.of),
            Opcode::BLT => cpu.blt(self.of),
            Opcode::BGT => cpu.bgt(self.of),
            Opcode::BLE => cpu.ble(self.of),
        }
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![InstructionParam::BranchOffset(self.of.into())],
            kind: InstructionKind::thumb(16),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_branching() {
        let asm = r"
            main:
                mov r1, #5 ; 0
                cmp r1, #5 ; 2
                beq branch ; 4 (offset is 0)
                mov r2, #1 ; 6

            branch:
                mov r2, #2 ; 8
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 2)
            .run(4)
    }

    #[test]
    fn test_offset_branching() {
        let asm = r"
            main:
                mov r1, #5 ; 0
                cmp r1, #5 ; 2
                beq branch ; 4 (offset is 2)
                mov r2, #1 ; 6
                mov r3, #4 ; 8

            branch:
                mov r1, #3 ; 10
                mov r2, #2 ; 12
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(3, 0)
            .assert_reg(1, 3)
            .assert_reg(2, 2)
            .run(5)
    }
}
