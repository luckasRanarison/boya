use crate::cpu::isa::prelude::*;

/// Load/store sign-extended byte/halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  1 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    ro: u8,
    rb: u8,
    rd: u8,
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
    STRH,
    LDSB,
    LDRH,
    LDSH,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDSB,
            2 => Self::LDRH,
            3 => Self::LDSH,
            _ => unreachable!("invalid thumb 8 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = cpu.registers.get(self.ro, cpu.cpsr.op_mode());
        let offset = RegisterOffset::new(value, AddrMode::IB, false);

        match self.op {
            Opcode::STRH => cpu.strh(self.rd, self.rb, offset),
            Opcode::LDSB => cpu.ldsb(self.rd, self.rb, offset),
            Opcode::LDRH => cpu.ldrh(self.rd, self.rb, offset),
            Opcode::LDSH => cpu.ldsh(self.rd, self.rb, offset),
        }
    }

    fn get_data(&self) -> InstructionData {
        let offset = RegisterOffsetData::simple(self.rb, self.ro.reg());

        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![self.rd.reg().into(), offset.into()],
            kind: InstructionKind::thumb(8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lds() {
        let asm = r"
            mov   r0, #2
            lsl   r0, r0, #24 ; 0x0200_0000
            mov   r1, #1
            mov   r2, #5
            ldrsb r3, [r0, r1]
            ldrsh r4, [r0, r2]
        ";

        GbaTestBuilder::new()
            .thumb()
            .setup(|cpu| {
                cpu.bus.write_byte(0x0200_0001, -1_i8 as u8);
                cpu.bus.write_hword(0x0200_0005, -5_i16 as u16);
            })
            .asm(asm)
            .assert_reg(3, -1_i32 as u32)
            .assert_reg(4, -5_i32 as u32)
            .run(6);
    }
}
