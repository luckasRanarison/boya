use crate::arm7tdmi::isa::prelude::*;

/// Move shifted register
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |    Op   |         Offset5        |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format1 {
    op: Opcode,
    of: u8,
    rs: u8,
    rd: u8,
}

impl Debug for Format1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, {:?}, {:?}",
            self.op,
            self.rd.reg(),
            self.rs.reg(),
            self.of.imm()
        )
    }
}

impl From<u16> for Format1 {
    fn from(value: u16) -> Self {
        let op = value.get_bits(11, 12).into();
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

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0b00 => Self::LSL,
            0b01 => Self::LSR,
            0b10 => Self::ASR,
            value => unreachable!("invalid format 1 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Format1 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let nn = self.of.imm();

        match self.op {
            Opcode::LSL => cpu.lsl(self.rd, self.rs, nn),
            Opcode::LSR => cpu.lsr(self.rd, self.rs, nn),
            Opcode::ASR => cpu.asr(self.rd, self.rs, nn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_shift() {
        let asm = r"
            mov r1, 2
            lsl r2, r1, 2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 8)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(2);
    }

    #[test]
    fn test_arithmetic_shift() {
        let asm = r"
            mov r0, 0
            mvn r1, r0
            asr r2, r1, 1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, !0)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .run(3);
    }
}
