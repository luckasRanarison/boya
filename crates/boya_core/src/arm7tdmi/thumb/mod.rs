mod formats;

pub use formats::InstructionFormat;

use formats::*;

use crate::{bus::Bus, utils::bitflags::Bitflag};

use super::{Arm7tdmi, common::ToOperand};

impl<B: Bus> Arm7tdmi<B> {
    pub fn decode_thumb(&self, word: u32) -> InstructionFormat {
        let word_aligned = self.pc() & 0b1 == 0;
        let (lsb, msb) = if word_aligned { (0, 15) } else { (16, 31) };
        let instruction = word.get_bits(lsb, msb) as u16;

        if instruction.get_bits(11, 15) == 0b00011 {
            InstructionFormat::Format2(Format2::from(instruction))
        } else if instruction.get_bits(13, 15) == 0b000 {
            InstructionFormat::Format1(Format1::from(instruction))
        } else if instruction.get_bits(13, 15) == 0b001 {
            InstructionFormat::Format3(Format3::from(instruction))
        } else if instruction.get_bits(10, 15) == 0b010000 {
            InstructionFormat::Format4(Format4::from(instruction))
        } else if instruction.get_bits(10, 15) == 0b010001 {
            InstructionFormat::Format5(Format5::from(instruction))
        } else if instruction.get_bits(11, 15) == 0b01001 {
            InstructionFormat::Format6(Format6::from(instruction))
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
            InstructionFormat::Format5(op) => self.exec_thumb_format5(op),
            InstructionFormat::Format6(op) => self.exec_thumb_format6(op),
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
            Opcode2::ADD => self.add(op.rs, op.nn, op.rd, true),
            Opcode2::SUB => self.sub(op.rs, op.nn, op.rd),
        }
    }

    pub fn exec_thumb_format3(&mut self, op: Format3) {
        let nn = op.nn.immediate();

        match op.opcode {
            Opcode3::MOV => self.mov(op.rd, nn, true),
            Opcode3::CMP => self.cmp(op.rd, nn),
            Opcode3::ADD => self.add(op.rd, nn, op.rd, true),
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

    pub fn exec_thumb_format5(&mut self, op: Format5) {
        match op.opcode {
            Opcode5::ADD => self.add(op.rd, op.rs.register(), op.rd, false),
            Opcode5::CMP => self.cmp(op.rd, op.rs.register()),
            Opcode5::MOV => self.mov(op.rd, op.rs.register(), false),
            Opcode5::BX => self.bx(op.rs),
        }
    }

    pub fn exec_thumb_format6(&mut self, op: Format6) {
        let address = self.pc() + op.nn as u32;
        let word = self.bus.read_u32(address);
        println!("address: {address}, word: {word}");
        self.set_reg(op.rd, word);
    }
}

#[cfg(test)]
mod tests {
    use crate::{arm7tdmi::psr::Psr, bus::Bus, test::AsmTestBuilder};

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
            mov r2, 2
            mov r3, 3
            orr r1, r0
            bic r3, r2
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(1, 3)
            .assert_reg(3, 1)
            .assert_flag(Psr::Z, false)
            .assert_flag(Psr::N, false)
            .run(6);
    }

    #[test]
    fn test_mul_basic() {
        let asm = r"
            mov r0, 2
            mov r1, 3
            sub r2, r1, r0 ; sets carry
            mul r0, r1
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 6)
            .assert_flag(Psr::C, false)
            .run(4);
    }

    // #[test]
    // fn test_bx() {
    //     let asm = r"
    //         start:
    //             mov r0, 13 ; 0b1101
    //             bx  r0
    //
    //         target EQU 13
    //             mov r1, 2
    //     ";
    //
    //     AsmTestBuilder::new()
    //         .thumb()
    //         .asm(asm)
    //         .assert_reg(1, 2)
    //         .assert_flag(Psr::T, true)
    //         .run(3);
    // }

    #[test]
    fn test_ldr_offset() {
        let asm = r"
            ldr r1, [PC, #16]
        ";

        AsmTestBuilder::new()
            .thumb()
            .setup(|cpu| cpu.bus.write_u32(20, 5))
            .asm(asm)
            .assert_reg(1, 5)
            .run(1);
    }
}
