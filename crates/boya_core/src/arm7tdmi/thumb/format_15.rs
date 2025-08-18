use crate::arm7tdmi::common::format_rlist;

use super::prelude::*;

/// Multiple load/store
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  0 |  L |      Rb      |                 RList                 |
/// +-------------------------------------------------------------------------------+
pub struct Format15 {
    op: Opcode,
    rb: u8,
    rlist: u8,
}

impl Debug for Format15 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}!, {}",
            self.op,
            self.rb.reg(),
            format_rlist(self.rlist, None)
        )
    }
}

impl From<u16> for Format15 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get(11));
        let rb = value.get_bits_u8(8, 10);
        let rlist = value.get_bits_u8(0, 7);

        Self { op, rb, rlist }
    }
}

#[derive(Debug)]
enum Opcode {
    STMIA,
    LDMIA,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STMIA,
            1 => Self::LDMIA,
            _ => unreachable!("invalid format 15 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format15(&mut self, instr: Format15) {
        match instr.op {
            Opcode::STMIA => self.stmia(instr.rlist, instr.rb),
            Opcode::LDMIA => self.ldmia(instr.rlist, instr.rb),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stmia() {
        let asm = r"
            mov   r0, 24
            mov   r1, 1
            mov   r2, 2
            mov   r3, 3
            stmia r0!, {r1,r2,r3}
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 36)
            .assert_word(24, 1)
            .assert_word(28, 2)
            .assert_word(32, 3)
            .run(5)
    }

    #[test]
    fn test_ldmia() {
        let asm = r"
            mov   r0, 24
            ldmia r0!, {r1,r2,r3}
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.write_word(24, 1);
                cpu.bus.write_word(28, 2);
                cpu.bus.write_word(32, 3);
            })
            .assert_reg(0, 36)
            .assert_reg(1, 1)
            .assert_reg(2, 2)
            .assert_reg(3, 3)
            .run(2)
    }
}
