use crate::cpu::isa::prelude::*;

/// Load/store halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  0 |  0 | Op |           Offset5      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    nn: u16,
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
            self.nn.imm()
        )
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_u8(11).into();
        let nn = value.get_bits(6, 10) << 1;
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, nn, rb, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STRH,
    LDRH,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDRH,
            _ => unreachable!("invalid thumb 10 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = self.nn.into();
        let offset = RegisterOffset::new(value, AddrMode::IB, false);

        match self.op {
            Opcode::STRH => cpu.strh(self.rd, self.rb, offset),
            Opcode::LDRH => cpu.ldrh(self.rd, self.rb, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldr_word() {
        let asm = r"
            mov   r0, #2
            lsl   r0, r0, #24 ; 0x0200_0000
            ldrh  r1, [r0, #8]
            strh  r1, [r0, #4]
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .setup(|cpu| cpu.bus.write_hword(0x0200_0008, 420))
            .assert_reg(1, 420)
            .assert_hword(0x0200_0004, 420)
            .run(4);
    }
}
