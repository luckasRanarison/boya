use super::prelude::*;

/// ALU operations
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  0 |  0 |  0 |         Op        |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format4 {
    op: Opcode,
    rs: u8,
    rd: u8,
}

impl Debug for Format4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}, {:?}", self.op, self.rd.reg(), self.rs.reg())
    }
}

impl From<u16> for Format4 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(6, 9));
        let rs = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, rs, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    AND,
    EOR,
    LSL,
    LSR,
    ASR,
    ADC,
    SBC,
    ROR,
    TST,
    NEG,
    CMP,
    CMN,
    ORR,
    MUL,
    BIC,
    MVN,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0x0 => Self::AND,
            0x1 => Self::EOR,
            0x2 => Self::LSL,
            0x3 => Self::LSR,
            0x4 => Self::ASR,
            0x5 => Self::ADC,
            0x6 => Self::SBC,
            0x7 => Self::ROR,
            0x8 => Self::TST,
            0x9 => Self::NEG,
            0xA => Self::CMP,
            0xB => Self::CMN,
            0xC => Self::ORR,
            0xD => Self::MUL,
            0xE => Self::BIC,
            0xF => Self::MVN,
            _ => unreachable!("invalid format 4 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format4(&mut self, instr: Format4) {
        match instr.op {
            Opcode::AND => self.and(instr.rd, instr.rs),
            Opcode::EOR => self.eor(instr.rd, instr.rs),
            Opcode::LSL => self.lsl(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::LSR => self.lsr(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::ASR => self.asr(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::ADC => self.adc(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::SBC => self.sbc(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::ROR => self.ror(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::TST => self.tst(instr.rd, instr.rs),
            Opcode::NEG => self.neg(instr.rd, instr.rs),
            Opcode::CMP => self.cmp(instr.rd, instr.rs.reg()),
            Opcode::CMN => self.cmn(instr.rd, instr.rs.reg()),
            Opcode::ORR => self.orr(instr.rd, instr.rs),
            Opcode::MUL => self.mul(instr.rd, instr.rs.reg(), instr.rd),
            Opcode::BIC => self.bic(instr.rd, instr.rs),
            Opcode::MVN => self.mvn(instr.rd, instr.rs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_carry() {
        let asm = r"
            mov r0, 5
            sub r1, r0, 2
            adc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 9)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_sub_carry() {
        let asm = r"
            mov r0, 5
            mov r1, 1
            sbc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 3)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_negation() {
        let asm = r"
            mov r0, 2
            neg r1, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, -2i32 as u32)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .assert_flag(Psr::V, false)
            .run(2);
    }

    #[test]
    fn test_logical_op() {
        let asm = r"
            mov r0, 2
            mov r1, 1
            mov r2, 2
            mov r3, 3
            orr r1, r0
            bic r3, r2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 3)
            .assert_reg(3, 1)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(6);
    }

    #[test]
    fn test_mul_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            sub r2, r1, r0 ; sets carry
            mul r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 6)
            .assert_flag(Psr::C, false)
            .run(4);
    }
}
