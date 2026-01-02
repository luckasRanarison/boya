use crate::cpu::isa::prelude::*;

/// Multiple load/store
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  0 |  L |      Rb      |                 RList                 |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    rb: u8,
    rlist: u8,
}

impl Debug for Instruction {
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

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_u8(11).into();
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

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STMIA,
            1 => Self::LDMIA,
            _ => unreachable!("invalid thumb 15 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::STMIA => cpu.stm(self.rlist.into(), self.rb, AddrMode::IA, true, false),
            Opcode::LDMIA => cpu.ldm(self.rlist.into(), self.rb, AddrMode::IA, true, false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stmia() {
        let asm = r"
            mov   r0, #2
            lsl   r0, #24
            mov   r1, #1
            mov   r2, #2
            mov   r3, #3
            stmia r0!, {r1,r2,r3}
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0x0200_000C)
            .assert_word(0x0200_0000, 1)
            .assert_word(0x0200_0004, 2)
            .assert_word(0x0200_0008, 3)
            .run(6)
    }

    #[test]
    fn test_ldmia() {
        let asm = r"
            mov   r0, #2
            lsl   r0, r0, #24 ; 0x0200_0000
            ldmia r0!, {r1,r2,r3}
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| {
                cpu.bus.write_word(0x0200_0000, 1);
                cpu.bus.write_word(0x0200_0004, 2);
                cpu.bus.write_word(0x0200_0008, 3);
            })
            .assert_reg(0, 0x0200_000C)
            .assert_reg(1, 1)
            .assert_reg(2, 2)
            .assert_reg(3, 3)
            .run(3)
    }
}
