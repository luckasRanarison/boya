use crate::arm7tdmi::isa::prelude::*;

/// PSR transfer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0|I|1|0|P|O|O| Field |   Rd  |       Operand          |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    psr: PsrKind,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Instruction { cd, psr, .. } = self;

        match &self.op {
            Opcode::MRS { rd } => write!(f, "MRS{cd:?} {rd:?}, {psr:?}"),
            Opcode::MSR { fd, op } => write!(f, "MSR{cd:?} {psr:?}_{fd:?}, {op:?}"),
        }
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let op = value.into();

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
        let shift = value.get_bits(8, 11) << 1;
        let imm = value.get_bits(0, 7);

        imm.rotate_right(shift).imm()
    } else {
        value.get_bits_u8(0, 3).reg()
    }
}

impl<B: Bus> Executable<B> for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        match self.op {
            Opcode::MRS { rd } => cpu.store_psr_op(rd, self.psr),
            Opcode::MSR { fd, op } => cpu.update_psr_op(op, fd.mask, self.psr),
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
            MSR    CPSR_C, 11000000_00000000_00000000_00010001b
            MSR    CPSR_FS, 00100000_00000000_00000000_00000001b
            MRS    R2, CPSR
            MSR    SPSR_FSXC, R2
        ";

        // SVC mode on boot
        AsmTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 0b00000000_00000000_00000000_11010011)
            .assert_reg(2, 0b00100000_00000000_00000000_00010001)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::C, true)
            .assert_fn(|cpu| {
                let op_mode = cpu.cpsr.operating_mode();
                let spsr = cpu.bank.get_spsr(op_mode);

                assert_eq!(op_mode, OperatingMode::FIQ);
                assert_eq!(spsr.value(), 0b00100000_00000000_00000000_00010001);
            })
            .run(5);
    }
}
