use crate::cpu::isa::prelude::*;

/// Single data transfer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 1|I|P|U|B|W|L|   Rn  |   Rd  |        Offset          |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    amod: AddrMode,
    rn: u8,
    rd: u8,
    b: bool,
    wb: bool,
    of: Operand,
}

impl From<Instruction> for DebuggableInstruction {
    fn from(value: Instruction) -> Self {
        let offset = DebuggableRegisterOffset {
            amod: value.amod,
            base: RegisterOffsetBase::Plain(value.rn),
            offset: Some(value.of.clone()),
            wb: value.wb,
        };

        Self {
            keyword: format!("{:?}{}", value.op, if value.b { "B" } else { "" }),
            args: vec![value.rd.reg().into(), offset.into()],
            kind: InstructionKind::arm(9, value.cd, None),
            source: Box::new(value),
        }
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let p = value.get_u8(24);
        let u = value.get_u8(23);
        let b = value.has(22);
        let amod = AddrMode::new(p, u);
        let wb = value.has(21);
        let op = value.get_u8(20).into();
        let rn = value.get_bits_u8(16, 19);
        let rd = value.get_bits_u8(12, 15);

        let of = if value.has(25) {
            let kind = ShiftKind::from(value.get_bits_u8(5, 6));
            let shift = Shift::imm(value.get_bits_u8(7, 11), kind);
            let rm = value.get_bits_u8(0, 3).reg();

            rm.shift(shift)
        } else {
            value.get_bits(0, 11).imm()
        };

        Self {
            cd,
            op,
            amod,
            wb,
            b,
            rn,
            rd,
            of,
        }
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
            0x0 => Self::STR,
            0x1 => Self::LDR,
            _ => unreachable!("invalid arm 9 opcode: {value:#b}"),
        }
    }
}

impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        let value = cpu.get_operand(self.of);
        let offset = RegisterOffset::new(value, self.amod, self.wb);

        match self.op {
            Opcode::STR if self.b => cpu.strb(self.rd, self.rn, offset),
            Opcode::LDR if self.b => cpu.ldrb(self.rd, self.rn, offset),
            Opcode::STR => cpu.str(self.rd, self.rn, offset),
            Opcode::LDR => cpu.ldr(self.rd, self.rn, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_data_transfer() {
        let asm = r"
            MOV     R0, #5 ; 0
            MOV     R1, 0x0200_0000
            MOV     R2, 0xF
            STRB    R0, [R1, R2, LSL #8]
            LDR     R3, [R1, 0xF00]
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_byte(0x0200_0F00, 5)
            .assert_reg(3, 5)
            .run(5)
    }
}
