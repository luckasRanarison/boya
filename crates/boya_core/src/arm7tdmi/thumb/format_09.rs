use crate::arm7tdmi::isa::prelude::*;

/// Load/store with immediate offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  1 |    Op   |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format9 {
    op: Opcode,
    nn: u16, // 0-31 for BYTE, 0-124 for WORD
    rb: u8,
    rd: u8,
}

impl Debug for Format9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, [{:?}, {:?}]",
            self.op,
            self.rd.reg(),
            self.rb.reg(),
            self.nn.imm()
        )
    }
}

impl From<u16> for Format9 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(11, 12));
        let is_word = matches!(op, Opcode::STR | Opcode::LDR);
        let nn = value.get_bits(6, 10);
        let nn = if is_word { nn << 2 } else { nn };
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, nn, rb, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STR,
    LDR,
    STRB,
    LDRB,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::LDR,
            2 => Self::STRB,
            3 => Self::LDRB,
            _ => unreachable!("invalid format 9 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Format9 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let addr = cpu.get_reg(self.rb) + self.nn as u32;

        match self.op {
            Opcode::STR => cpu.str(self.rd, addr),
            Opcode::LDR => cpu.ldr(self.rd, addr),
            Opcode::STRB => cpu.strb(self.rd, addr),
            Opcode::LDRB => cpu.ldrb(self.rd, addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_immediate() {
        let asm = r"
            mov  r0, 7
            ldr  r1, [r0, 116]
            strb r1, [r0, 23]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| cpu.bus.write_word(123, 5))
            .assert_reg(1, 5)
            .assert_byte(30, 5)
            .run(3);
    }
}
