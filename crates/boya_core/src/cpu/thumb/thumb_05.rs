pub use crate::cpu::isa::prelude::*;

/// Hi register operations/branch exchange
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  0 |  1 |    Op   | Hd | Hs |     Rs/Hs    |     Rd/Hd    |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    rs: u8,
    rd: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(8, 9).into();
        let msbd = value.get_u8(7);
        let msbs = value.get_u8(6);
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self {
            op,
            rs: rs | (msbs << 3),
            rd: rd | (msbd << 3),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    ADD,
    CMP,
    MOV,
    BX,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ADD,
            1 => Self::CMP,
            2 => Self::MOV,
            3 => Self::BX,
            _ => unreachable!("invalid thumb 5 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::ADD => cpu.add(self.rd, self.rd, self.rs.reg(), false),
            Opcode::CMP => cpu.cmp(self.rd, self.rs.reg(), true),
            Opcode::MOV => cpu.mov(self.rd, self.rs.reg(), false),
            Opcode::BX => cpu.bx(self.rs),
        }
    }

    fn get_data(&self) -> InstructionData {
        let args = match &self.op {
            Opcode::BX => vec![self.rs.reg().into()],
            _ => vec![self.rd.reg().into(), self.rs.reg().into()],
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            kind: InstructionKind::thumb(5),
            args,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hi_reg_ops() {
        let asm = r"
            mov r0, #24
            mov pc, r0
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(15, 28) // pre-fetch + 4
            .run(2);
    }

    #[test]
    fn test_bx() {
        let asm = r"
            mov r0, #24
            bx  r0
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_flag(Psr::T, false)
            .assert_reg(15, 32) // pre-fetch + 8
            .run(2);
    }
}
