use crate::cpu::isa::prelude::*;

/// Block data transfer
/// +-----------------------------------------------------------------+
/// |...3 ..................2 ..................1 ..................0.|
/// |-----------------------------------------------------------------|
/// |_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_|
/// |-----------------------------------------------------------------|
/// |  Cond  |1 0 0|P|U|S|W|L|   Rn  |              RList             |
/// +-----------------------------------------------------------------+
pub struct Instruction {
    cd: Condition,
    op: Opcode,
    s: bool,
    amod: AddrMode,
    wb: bool,
    rn: u8,
    rlist: u16,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?} {:?}{}, {:?}{}",
            self.op,
            self.amod,
            self.cd,
            self.rn.reg(),
            if self.wb { "!" } else { "" },
            format_rlist(self.rlist, None),
            if self.s { "^" } else { "" }
        )
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let cd = value.get_bits_u8(28, 31).into();
        let p = value.get_u8(24);
        let u = value.get_u8(23);
        let s = value.has(22);
        let wb = value.has(21);
        let op = value.get_u8(20).into();
        let rn = value.get_bits_u8(16, 19);
        let rlist = value.get_bits(0, 15) as u16;
        let amod = AddrMode::new(p, u);

        Self {
            cd,
            op,
            s,
            amod,
            wb,
            rn,
            rlist,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    STM,
    LDM,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::STM,
            0x1 => Self::LDM,
            _ => unreachable!("invalid arm 11 opcode: {value:#b}"),
        }
    }
}

impl Executable for Instruction {
    fn condition(&self) -> Condition {
        self.cd
    }

    fn dispatch(self, cpu: &mut Arm7tdmi) -> Cycle {
        match self.op {
            Opcode::STM => cpu.stm(self.rlist, self.rn, self.amod, self.wb, self.s),
            Opcode::LDM => {
                if self.s && self.rlist.has(15) {
                    cpu.restore_cpsr();
                }

                cpu.ldm(self.rlist, self.rn, self.amod, self.wb, self.s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_data_transfer() {
        let asm = r"
            MOV     R0, #1 ; 0
            MOV     R1, #2 ; 4
            STMIA   R13!, {R0, R1, R15} ; 8
        ";

        AsmTestBuilder::new()
            .asm(asm)
            .assert_word(SP_START, 1)
            .assert_word(SP_START + 4, 2)
            .assert_word(SP_START + 8, ARM_MAIN_START + 20) // 8 + 12
            .assert_reg(13, SP_START + 12)
            .run(3)
    }
}
