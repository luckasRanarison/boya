pub use super::prelude::*;

/// Hi register operations/branch exchange
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  0 |  1 |    Op   | Hd | Hs |     Rs/Hs    |     Rd/Hd    |
/// +-------------------------------------------------------------------------------+
pub struct Format5 {
    op: Opcode,
    rs: u8,
    rd: u8,
}

impl Debug for Format5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.op {
            Opcode::BX => write!(f, "BX {:?}", self.rs.reg()),
            opcode => write!(f, "{opcode:?} {:?}, {:?}", self.rd.reg(), self.rs.reg()),
        }
    }
}

impl From<u16> for Format5 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(8, 9));
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

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::ADD,
            1 => Self::CMP,
            2 => Self::MOV,
            3 => Self::BX,
            _ => unreachable!("invalid format 5 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format5(&mut self, instr: Format5) {
        match instr.op {
            Opcode::ADD => self.add(instr.rd, instr.rs.reg(), instr.rd, false),
            Opcode::CMP => self.cmp(instr.rd, instr.rs.reg()),
            Opcode::MOV => self.mov(instr.rd, instr.rs.reg(), false),
            Opcode::BX => self.bx(instr.rs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hi_reg_ops() {
        let asm = r"
            mov r0, 24
            mov pc, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(15, 28) // pre-fetch + 4
            .run(2);
    }
}
