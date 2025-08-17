use super::prelude::*;

/// Add/Substract
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  0 |  0 |  1 |  1 |  I | Op |  Rn/Offset3  |      Rs      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format2 {
    op: Opcode,
    nn: Operand,
    rs: u8,
    rd: u8,
}

impl Debug for Format2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, {:?}, {:?}",
            self.op,
            self.rd.reg(),
            self.rs.reg(),
            self.nn,
        )
    }
}

impl From<u16> for Format2 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(9, 10));
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

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0b00 | 0b10 => Self::ADD,
            0b01 | 0b11 => Self::SUB,
            value => unreachable!("invalid format 2 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format2(&mut self, instr: Format2) {
        match instr.op {
            Opcode::ADD => self.add(instr.rs, instr.nn, instr.rd, true),
            Opcode::SUB => self.sub(instr.rs, instr.nn, instr.rd, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sub_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            add r2, r1, r0
            add r3, r2, 3 
            sub r4, r3, r0
            cmp r4, 6
        ";

        AsmTestBuilder::new()
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
