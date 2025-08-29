use crate::arm7tdmi::isa::prelude::*;

/// Halfword and Signed data transer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0|P|U|I|W|L|  Rn   |  Rd   |0 0 0 0 1|S|H|1|  Rm    |
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0 0|P|U|I|W|L|  Rn   |  Rd   | Offset|1|S|H|1| Offset |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    cd: Condition,
    amod: AddrMode,
    wb: bool,
    rd: u8,
    rn: u8,
    of: Operand,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = format!("{:?}", self.op);
        let (prefix, suffix) = op.split_at(3);
        let op_cd = format!("{}{:?}{}", prefix, self.cd, suffix);
        let rn = self.rn.reg();

        write!(f, "{op_cd} {:?}, ", self.rd.reg())?;

        match self.amod {
            AddrMode::IB | AddrMode::DB if self.of.is_imm() && self.of.value == 0 => {
                write!(f, "[{rn:?}]")
            }
            AddrMode::IB => write!(f, "[{rn:?}, {:?}]", self.of),
            AddrMode::DB if self.of.is_imm() => write!(f, "[{rn:?}, #-{:?}]", self.of.value),
            AddrMode::DB => write!(f, "[{rn:?}, -{:?}]", self.of),
            AddrMode::IA => write!(f, "[{rn:?}], {:?}", self.of),
            AddrMode::DA if self.of.is_imm() => write!(f, "[{rn:?}], #-{:?}", self.of.value),
            AddrMode::DA => write!(f, "[{rn:?}], -{:?}", self.of),
        }?;

        if self.wb && matches!(self.amod, AddrMode::IB | AddrMode::DB) {
            write!(f, "!")?;
        }

        Ok(())
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let p = value.get_u8(24);
        let u = value.get_u8(23);
        let wb = value.has(22) || p == 0;
        let rn = value.get_bits_u8(16, 19);
        let rd = value.get_bits_u8(12, 15);
        let op = value.into();
        let amod = AddrMode::new(p, u);

        let of = match value.has(22) {
            true => ((value.get_bits(8, 11) << 4) | (value.get_bits(0, 3))).imm(),
            false => value.get_bits(0, 3).reg(),
        };

        Self {
            cd,
            amod,
            wb,
            rd,
            rn,
            op,
            of,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    STRH,
    LDRH,
    LDSB,
    LDSH,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match (value.get(20), value.get_bits(5, 6)) {
            (0, 0x1) => Self::STRH,
            (1, 0x1) => Self::LDRH,
            (1, 0x2) => Self::LDSB,
            (1, 0x3) => Self::LDSH,
            (_, op) => unreachable!("invalid format 7 opcode: {op:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        let value = cpu.get_operand(self.of);
        let offset = RegisterOffset::new(value, self.amod, self.wb);

        match self.op {
            Opcode::STRH => cpu.strh(self.rd, self.rn, offset),
            Opcode::LDRH => cpu.ldrh(self.rd, self.rn, offset),
            Opcode::LDSB => cpu.ldsb(self.rd, self.rn, offset),
            Opcode::LDSH => cpu.ldsh(self.rd, self.rn, offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hword_signed_transfer() {
        let asm = r"
            MOV     R0, 3
            MOV     R1, 5
            MOV     R2, 50
            STRH    R0, [R1, #45]!
            LDRH    R3, [R2], -R0
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_hword(50, 3)
            .assert_reg(1, 50)
            .assert_reg(2, 47)
            .assert_reg(3, 3)
            .run(5);
    }
}
