use crate::arm7tdmi::isa::prelude::*;

/// Load/store SP-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  1 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    nn: u16,
    rd: u8,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} R{:?}, [SP, {:?}]",
            self.op,
            self.rd.reg(),
            self.nn.imm()
        )
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_u8(11).into();
        let rd = value.get_bits_u8(8, 10);
        let nn = value.get_bits(0, 7) << 2;

        Self { op, nn, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STR,
    LDR,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STR,
            1 => Self::LDR,
            _ => unreachable!("invalid thumb 11 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let value = self.nn.into();
        let offset = RegisterOffset::new(value, AddrMode::IB, false);
        let sp = NamedRegister::SP as u8;

        match self.op {
            Opcode::STR => cpu.str(self.rd, sp, offset),
            Opcode::LDR => cpu.ldr(self.rd, sp, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_sp_relative() {
        let asm = r"
            mov r0, #25
            str r0, [SP, #12]
            ldr r2, [SP, #24]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| {
                cpu.set_reg(13_usize, 48);
                cpu.bus.write_word(72, 9);
            })
            .assert_byte(60, 25)
            .assert_reg(2, 9)
            .run(3);
    }
}
