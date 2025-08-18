use super::prelude::*;

/// Conditional branch
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  1 |        Cond       |               SOffset8                |
/// +-------------------------------------------------------------------------------+
pub struct Format16 {
    op: Opcode,
    of: i16,
}

impl Debug for Format16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.op, self.of)
    }
}

impl From<u16> for Format16 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(8, 11));
        let of = (value.get_bits(0, 7) << 1) as i16;

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

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
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
            _ => unreachable!("invalid format 16 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format16(&mut self, instr: Format16) {
        match instr.op {
            Opcode::BEQ => self.beq(instr.of),
            Opcode::BNE => self.bne(instr.of),
            Opcode::BCS => self.bcs(instr.of),
            Opcode::BCC => self.bcc(instr.of),
            Opcode::BMI => self.bmi(instr.of),
            Opcode::BPL => self.bpl(instr.of),
            Opcode::BVS => self.bvs(instr.of),
            Opcode::BVC => self.bvc(instr.of),
            Opcode::BHI => self.bhi(instr.of),
            Opcode::BLS => self.bls(instr.of),
            Opcode::BGE => self.bge(instr.of),
            Opcode::BLT => self.blt(instr.of),
            Opcode::BGT => self.bgt(instr.of),
            Opcode::BLE => self.ble(instr.of),
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
                mov r2, #2 ; 12
        ";

        AsmTestBuilder::new()
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

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(3, 0)
            .assert_reg(1, 3)
            .assert_reg(2, 2)
            .run(5)
    }
}
