mod formats;

use formats::*;

use crate::{bus::Bus, utils::bitflags::Bitflag};

use super::{Arm7tdmi, common::ToOperand};

impl<B: Bus> Arm7tdmi<B> {
    pub fn fetch_thumb(&mut self) -> u16 {
        let word = self.bus.read_u32(self.pc());

        let instruction = match self.pc() % 2 == 0 {
            false => (word & 0xFFFF0000) >> 16,
            true => word & 0xFFFF,
        };

        self.increment_pc(2);

        instruction as u16
    }

    pub fn decode_thumb(&self, instruction: u16) -> InstructionFormat {
        if instruction.get_bits(11, 15) == 0b00011 {
            InstructionFormat::Format2(Format2::from(instruction))
        } else if instruction.get_bits(13, 15) == 0b000 {
            InstructionFormat::Format1(Format1::from(instruction))
        } else if instruction.get_bits(13, 15) == 0b001 {
            InstructionFormat::Format3(Format3::from(instruction))
        } else if instruction.get_bits(10, 15) == 0b010000 {
            InstructionFormat::Format4(Format4::from(instruction))
        } else {
            todo!()
        }
    }

    pub fn exec_thumb(&mut self, instruction: InstructionFormat) {
        match instruction {
            InstructionFormat::Format1(op) => self.exec_thumb_format1(op),
            InstructionFormat::Format2(op) => self.exec_thumb_format2(op),
            InstructionFormat::Format3(op) => self.exec_thumb_format3(op),
            InstructionFormat::Format4(op) => self.exec_thumb_format4(op),
        }
    }

    pub fn exec_thumb_format1(&mut self, op: Format1) {
        let nn = op.offset.immediate();

        match op.opcode {
            Opcode1::LSL => self.lsl(op.rs, nn, op.rd),
            Opcode1::LSR => self.lsr(op.rs, nn, op.rd),
            Opcode1::ASR => self.asr(op.rs, nn, op.rd),
        }
    }

    pub fn exec_thumb_format2(&mut self, op: Format2) {
        match op.opcode {
            Opcode2::ADD => self.add(op.rs, op.nn, op.rd),
            Opcode2::SUB => self.sub(op.rs, op.nn, op.rd),
        }
    }

    pub fn exec_thumb_format3(&mut self, op: Format3) {
        let nn = op.nn.immediate();

        match op.opcode {
            Opcode3::MOV => self.mov(op.rd, nn),
            Opcode3::CMP => self.cmp(op.rd, nn),
            Opcode3::ADD => self.add(op.rd, nn, op.rd),
            Opcode3::SUB => self.sub(op.rd, nn, op.rd),
        }
    }

    pub fn exec_thumb_format4(&mut self, op: Format4) {
        match op.opcode {
            Opcode4::AND => self.and(op.rd, op.rs),
            Opcode4::EOR => self.eor(op.rd, op.rs),
            Opcode4::LSL => self.lsl(op.rd, op.rs.register(), op.rd),
            Opcode4::LSR => self.lsr(op.rd, op.rs.register(), op.rd),
            Opcode4::ASR => self.asr(op.rd, op.rs.register(), op.rd),
            Opcode4::ADC => self.adc(op.rd, op.rs.register(), op.rd),
            Opcode4::SBC => self.sbc(op.rd, op.rs.register(), op.rd),
            Opcode4::ROR => self.ror(op.rd, op.rs.register(), op.rd),
            Opcode4::TST => self.tst(op.rd, op.rs),
            Opcode4::NEG => self.neg(op.rd, op.rs),
            Opcode4::CMP => self.cmp(op.rd, op.rs.register()),
            Opcode4::CMN => self.cmn(op.rd, op.rs.register()),
            Opcode4::ORR => self.orr(op.rd, op.rs),
            Opcode4::MUL => self.mul(op.rd, op.rs.register(), op.rd),
            Opcode4::BIC => self.bic(op.rd, op.rs),
            Opcode4::MVN => self.mvn(op.rd, op.rs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arm7tdmi::psr::Psr, test::AsmTestBuilder};

    #[test]
    fn test_move() {
        let asm = r"
            mov r1, 5
            mvn r2, r1
            mov r3, 0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 5)
            .assert_reg(2, !5)
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .run(3);
    }

    #[test]
    fn test_logic_shift() {
        let asm = r"
            mov r1, 2
            lsl r2, r1, 2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 8)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(2);
    }

    #[test]
    fn test_arithmetic_shift() {
        let asm = r"
            mov r0, 0
            mvn r1, r0
            asr r2, r1, 1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, !0)
            .assert_flag(Psr::C, true)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .run(3);
    }

    #[test]
    fn test_add_sub_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            add r2, r1, r0
            add r3, r2, 3 
            sub r4, r3, r0
            cmp r4, 6
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(2, 5)
            .assert_reg(3, 8)
            .assert_reg(4, 6)
            .assert_flag(Psr::C, true) // no borrow
            .assert_flag(Psr::Z, true)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(6);
    }

    #[test]
    fn test_add_carry() {
        let asm = r"
            mov r0, 5
            sub r1, r0, 2
            adc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 9)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_sub_carry() {
        let asm = r"
            mov r0, 5
            mov r1, 1
            sbc r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 3)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .assert_flag(Psr::V, false)
            .run(4);
    }

    #[test]
    fn test_negation() {
        let asm = r"
            mov r0, 2
            neg r1, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, -2i32 as u32)
            .assert_flag(Psr::C, false)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, true)
            .assert_flag(Psr::V, false)
            .run(2);
    }

    #[test]
    fn test_logical_op() {
        let asm = r"
            mov r0, 2
            mov r1, 1
            orr r1, r0
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 3)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(3);
    }
}
