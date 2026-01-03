use crate::cpu::isa::prelude::*;

/// Data processing
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |0 0|I|   Op  |S|  Rn   |  Rd   |       Operand2         |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    s: bool,
    rn: u8,
    rd: u8,
    op2: Operand,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let op = value.get_bits_u8(21, 24).into();
        let s = value.has(20);
        let rn = value.get_bits_u8(16, 19);
        let rd = value.get_bits_u8(12, 15);

        let op2 = if value.has(25) {
            let shift = value.get_bits_u8(8, 11) << 1;
            let nn = value.get_bits(0, 7).imm();

            match shift {
                0 => nn,
                _ => nn.shift(Shift::imm(shift, ShiftKind::ROR)),
            }
        } else {
            let sk = ShiftKind::from(value.get_bits_u8(5, 6));
            let rm = value.get_bits_u8(0, 3).reg();

            let shift = match value.get(4) {
                0 => Shift::imm(value.get_bits_u8(7, 11), sk),
                _ => Shift::reg(value.get_bits_u8(8, 11), sk),
            };

            rm.shift(shift)
        };

        Self {
            cd,
            op,
            s,
            rn,
            rd,
            op2,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::AND,
            0x1 => Self::EOR,
            0x2 => Self::SUB,
            0x3 => Self::RSB,
            0x4 => Self::ADD,
            0x5 => Self::ADC,
            0x6 => Self::SBC,
            0x7 => Self::RSC,
            0x8 => Self::TST,
            0x9 => Self::TEQ,
            0xA => Self::CMP,
            0xB => Self::CMN,
            0xC => Self::ORR,
            0xD => Self::MOV,
            0xE => Self::BIC,
            0xF => Self::MVN,
            _ => unreachable!("invalid arm 5 opcode: {value:#b}"),
        }
    }
}

impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        if self.s && self.rd.reg().is_pc() {
            cpu.restore_cpsr();
        }

        match self.op {
            Opcode::AND => cpu.and(self.rd, self.rn, self.op2, self.s),
            Opcode::EOR => cpu.eor(self.rd, self.rn, self.op2, self.s),
            Opcode::SUB => cpu.sub(self.rd, self.rn, self.op2, self.s),
            Opcode::RSB => cpu.rsb(self.rd, self.rn, self.op2, self.s),
            Opcode::ADD => cpu.add(self.rd, self.rn, self.op2, self.s),
            Opcode::ADC => cpu.adc(self.rd, self.rn, self.op2, self.s),
            Opcode::SBC => cpu.sbc(self.rd, self.rn, self.op2, self.s),
            Opcode::RSC => cpu.rsc(self.rd, self.rn, self.op2, self.s),
            Opcode::ORR => cpu.orr(self.rd, self.rn, self.op2, self.s),
            Opcode::BIC => cpu.bic(self.rd, self.rn, self.op2, self.s),
            Opcode::TST => cpu.tst(self.rn, self.op2, self.s),
            Opcode::TEQ => cpu.teq(self.rn, self.op2, self.s),
            Opcode::CMP => cpu.cmp(self.rn, self.op2, self.s),
            Opcode::CMN => cpu.cmn(self.rn, self.op2, self.s),
            Opcode::MOV => cpu.mov(self.rd, self.op2, self.s),
            Opcode::MVN => cpu.mvn(self.rd, self.op2, self.s),
        }
    }

    fn get_data(&self) -> InstructionData {
        let op2 = self.op2.clone().into();
        let rn = self.rn.reg().into();
        let rd = self.rd.reg().into();

        let args = match self.op {
            Opcode::TST | Opcode::TEQ | Opcode::CMP | Opcode::CMN => vec![rn, op2],
            Opcode::MOV | Opcode::MVN => vec![rd, op2],
            _ => vec![rd, rn, op2],
        };

        InstructionData {
            keyword: format!("{:?}", self.op),
            args,
            kind: InstructionKind::arm(5, self.cd.into(), self.s.into(), false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_2() {
        let asm = r"
            MOVS   R0, #2  
            MOVS   R1, #5  
            MOVS   R2, #0xFF000000 ; imm8 + ror
            MOVS   R3, R1, LSL #2
            MOVS   R4, R3, LSR R0
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(0, 2)
            .assert_reg(2, 0xFF000000)
            .assert_reg(3, 20)
            .assert_reg(4, 5)
            .run(5);
    }

    #[test]
    fn test_condition() {
        let asm = r"
            MOVS    R0, #0xFF000000
            MOVMI   R1, #1
            MOVPL   R1, #2
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 1)
            .assert_flag(Psr::N, true)
            .run(3);
    }

    #[test]
    fn test_logical_op() {
        let asm = r"
            MOV    R0, #110b
            AND    R1, R0, #101b
            ORR    R2, R0, R1
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 0b100)
            .assert_reg(2, 0b110)
            .run(3);
    }

    #[test]
    fn test_arithmetic_op() {
        let asm = r"
            MOV    R0, #5
            RSB    R1, R0, #8
            ADD    R2, R1, R0
            SUBS   R3, R2, R0
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 3)
            .assert_reg(2, 8)
            .assert_reg(3, 3)
            .run(4);
    }

    #[test]
    fn test_special_lsr_shift() {
        let asm = r"
            MVN    R0, #0
            MOVS   R1, R0, LSR #32 ; op2 = 0
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 0)
            .assert_flag(Psr::C, true)
            .run(2);
    }

    #[test]
    fn test_special_asr_shift() {
        let asm = r"
            MVN    R0, #0
            MOVS   R1, R0, ASR #32 ; op2 = 0xFFFF_FFFF
        ";

        GbaTestBuilder::new()
            .asm(asm)
            .assert_reg(1, 0xFFFF_FFFF)
            .assert_flag(Psr::C, true)
            .run(2);
    }
}
