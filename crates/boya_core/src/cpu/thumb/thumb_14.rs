use crate::cpu::isa::prelude::*;

/// Push/Pop registers
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  1 | Op |  1 |  0 |  R |                 RList                 |
/// +-------------------------------------------------------------------------------+
pub struct Instruction {
    op: Opcode,
    lrpc: bool,
    rlist: u8,
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let op = value.get_u8(11).into();
        let lrpc = value.has(8);
        let rlist = value.get_bits_u8(0, 7);

        Self { op, lrpc, rlist }
    }
}

#[derive(Debug)]
enum Opcode {
    PUSH,
    POP,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::PUSH,
            1 => Self::POP,
            _ => unreachable!("invalid thumb 14 opcode: {value:b}"),
        }
    }
}

impl Executable for Instruction {
    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::PUSH => cpu.push(self.rlist, self.lrpc),
            Opcode::POP => cpu.pop(self.rlist, self.lrpc),
        }
    }

    fn get_data(&self) -> InstructionData {
        InstructionData {
            keyword: format!("{:?}", self.op),
            args: vec![RegisterList::new(self.rlist.into()).into()],
            kind: InstructionKind::thumb(14),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_reg() {
        let asm = r"
            mov  r0, #64
            mov  r1, #13
            mov  r2, #25
            push {r0,r1,r2}
            pop  {r3,r4,r5}
        ";

        GbaTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_word(SP_START - 4, 25)
            .assert_word(SP_START - 8, 13)
            .assert_word(SP_START - 12, 64)
            .assert_reg(3, 64)
            .assert_reg(4, 13)
            .assert_reg(5, 25)
            .assert_reg(13, SP_START)
            .run(5)
    }
}
