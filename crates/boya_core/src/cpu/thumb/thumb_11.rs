use crate::cpu::isa::prelude::*;

/// Load/store SP-relative
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  1 | Op |      Rd      |                Offset8                |
/// +-------------------------------------------------------------------------------+
#[derive(Debug)]
pub struct Instruction {
    pub op: Opcode,
    pub nn: u16,
    pub rd: u8,
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
pub enum Opcode {
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

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
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

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| cpu.bus.write_word(SP_START + 24, 9))
            .assert_byte(SP_START + 12, 25)
            .assert_reg(2, 9)
            .run(3);
    }
}
