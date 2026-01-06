use crate::cpu::isa::prelude::*;

/// Load/store with immediate offset
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  1 |    Op   |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    op: Opcode,
    nn: u16, // 0-31 for BYTE, 0-124 for WORD
    rb: u8,
    rd: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_bits_u8(11, 12).into();
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

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::LDR,
            2 => Self::STRB,
            3 => Self::LDRB,
            _ => unreachable!("invalid thumb 9 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = self.nn.into();
        let offset = RegisterOffset::new(value, AddrMode::IB, false);

        match self.op {
            Opcode::STR => cpu.str(self.rd, self.rb, offset),
            Opcode::LDR => cpu.ldr(self.rd, self.rb, offset),
            Opcode::STRB => cpu.strb(self.rd, self.rb, offset),
            Opcode::LDRB => cpu.ldrb(self.rd, self.rb, offset),
        }
    }

    fn get_data(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.nn.imm());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(9),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_immediate() {
        let asm = r"
            mov  r0, #2
            lsl  r0, r0, #24 ; 0x0200_0000
            ldr  r1, [r0, 0x50]
            strb r1, [r0, 0x0A]
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| cpu.bus.write_word(0x0200_0050, 5))
            .assert_reg(1, 5)
            .assert_byte(0x0200_000A, 5)
            .run(4);
    }
}
