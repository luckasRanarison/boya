use crate::arm7tdmi::isa::prelude::*;

/// Load/Store with register offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  0 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    ro: u8,
    rb: u8,
    rd: u8,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, [{:?}, {:?}]",
            self.op,
            self.rd.reg(),
            self.rb.reg(),
            self.ro.reg()
        )
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(10, 11).into();
        let ro = value.get_bits_u8(6, 8);
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, ro, rb, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STR,
    STRB,
    LDR,
    LDRB,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::STRB,
            2 => Self::LDR,
            3 => Self::LDRB,
            _ => unreachable!("invalid thumb 7 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = cpu.get_reg(self.ro);
        let offset = RegisterOffset::new(value, AddrMode::IB, false);

        match self.op {
            Opcode::STR => cpu.str(self.rd, self.rb, offset),
            Opcode::STRB => cpu.strb(self.rd, self.rb, offset),
            Opcode::LDR => cpu.ldr(self.rd, self.rb, offset),
            Opcode::LDRB => cpu.ldrb(self.rd, self.rb, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_str_reg_offset() {
        let asm = r"
            mov r0, #3
            mov r1, #2
            lsl r1, r1, #24 ; 0x0200_0000
            mov r2, #8
            str r0, [r1, r2]
            ldr r3, [r1, r2]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_word(0x0200_0008, 3)
            .assert_reg(3, 3)
            .run(6);
    }
}
