use crate::arm7tdmi::isa::prelude::*;

/// PSR transfer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0|I|1|0|P|O|O| Field |   Rd  |       Operand          |
/// +-----------------------------------------------------------------+
pub struct Format2 {
    cd: Condition,
    op: Opcode,
    psr: PsrKind,
}

impl Debug for Format2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Format2 { cd, psr, .. } = self;

        match &self.op {
            Opcode::MRS { rd } => write!(f, "MRS{cd:?} {rd:?}, {psr:?}"),
            Opcode::MSR { fd, op } => write!(f, "MSR{cd:?} {psr:?}_{fd:?}, {op:?}"),
        }
    }
}

impl From<u32> for Format2 {
    fn from(value: u32) -> Self {
        let cd = Condition::from(value.get_bits_u8(28, 31));
        let op = Opcode::from(value);

        let psr = match value.get(22) {
            0 => PsrKind::CPSR,
            _ => PsrKind::SPSR,
        };

        Self { cd, psr, op }
    }
}

#[derive(Debug)]
enum Opcode {
    MRS { rd: u8 },
    MSR { fd: PsrField, op: Operand },
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        if value.get(21) == 0 {
            Self::MRS {
                rd: value.get_bits_u8(12, 15),
            }
        } else {
            let fd = PsrField::from(value.get_bits_u8(16, 19));
            let imm = value.has(25);
            let op = decode_operand(value, imm);

            Self::MSR { fd, op }
        }
    }
}

fn decode_operand(value: u32, imm: bool) -> Operand {
    if imm {
        value.get_bits_u8(0, 3).reg()
    } else {
        let shift = value.get_bits(8, 11);
        let imm = value.get_bits(0, 7);

        imm.rotate_right(shift).imm()
    }
}

impl<B: Bus> Executable<B> for Format2 {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        match self.op {
            Opcode::MRS { rd } => cpu.store_psr_op(rd, self.psr),
            Opcode::MSR { fd, op } => cpu.set_psr_op(op, fd.mask, self.psr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psr_transfer() {
        let asm = r"
            MRS    R1, CPSR
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 0b00000000_00000000_00000000_11010011)
            .run(1);
    }
}
