use crate::arm7tdmi::common::{NamedRegister, format_rlist};

use super::prelude::*;

/// Push/Pop registers
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  0 |  1 |  1 | Op |  1 |  0 |  R |                 RList                 |
/// +-------------------------------------------------------------------------------+
pub struct Format14 {
    op: Opcode,
    lrpc: bool,
    rlist: u8,
}

impl Debug for Format14 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rlist = match self.op {
            Opcode::PUSH => format_rlist(self.rlist, self.lrpc.then_some(NamedRegister::LR)),
            Opcode::POP => format_rlist(self.rlist, self.lrpc.then_some(NamedRegister::PC)),
        };

        write!(f, "{:?} {{{rlist}}}", self.op)
    }
}

impl From<u16> for Format14 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get(11));
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

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::PUSH,
            1 => Self::POP,
            _ => unreachable!("invalid format 14 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Executable<B> for Format14 {
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        match self.op {
            Opcode::PUSH => cpu.push(self.rlist, self.lrpc),
            Opcode::POP => cpu.pop(self.rlist, self.lrpc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_reg() {
        let asm = r"
            mov  r0, 64
            mov  r1, 13
            mov  r2, 25
            push {r0,r1,r2}
            pop  {r3,r4,r5}
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_word(196, 64)
            .assert_word(192, 13)
            .assert_word(188, 25)
            .assert_reg(3, 25)
            .assert_reg(4, 13)
            .assert_reg(5, 64)
            .assert_reg(13, 200)
            .run(5)
    }
}
