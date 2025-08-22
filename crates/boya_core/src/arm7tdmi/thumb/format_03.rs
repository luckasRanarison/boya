use crate::arm7tdmi::isa::prelude::*;

// Move/Compare/Add/Substract immediate
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  1 |    Op   |    Rd   |                   Offset8                  |
/// +-------------------------------------------------------------------------------+
pub struct Format3 {
    op: Opcode,
    rd: u8,
    nn: u8,
}

impl Debug for Format3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}, {:?}", self.op, self.rd.reg(), self.nn.imm())
    }
}

impl From<u16> for Format3 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(11, 12));
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits_u8(0, 7);

        Self { op, rd, nn }
    }
}

#[derive(Debug)]
enum Opcode {
    MOV,
    CMP,
    ADD,
    SUB,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0b00 => Self::MOV,
            0b01 => Self::CMP,
            0b10 => Self::ADD,
            0b11 => Self::SUB,
            _ => unreachable!("invalid format 3 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Format3 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let nn = self.nn.imm();

        match self.op {
            Opcode::MOV => cpu.mov(self.rd, nn, true),
            Opcode::CMP => cpu.cmp(self.rd, nn, true),
            Opcode::ADD => cpu.add(self.rd, self.rd, nn, true),
            Opcode::SUB => cpu.sub(self.rd, self.rd, nn, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let asm = r"
            mov r1, 5
            mvn r2, r1
            mov r3, 0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 5)
            .assert_reg(2, !5)
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .run(3);
    }
}
