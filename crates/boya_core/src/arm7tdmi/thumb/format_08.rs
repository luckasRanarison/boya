use super::prelude::*;

/// Load/store sign-extended byte/halfword
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  0 |  1 |  0 |  1 |    Op   |  1 |      Ro      |      Rb      |      Rd      |
/// +-------------------------------------------------------------------------------+
pub struct Format8 {
    op: Opcode,
    ro: u8,
    rb: u8,
    rd: u8,
}

impl Debug for Format8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}, [{:?}, {:?}]",
            self.op,
            self.rd.reg(),
            self.rb.reg(),
            self.ro.reg()
        )
    }
}

impl From<u16> for Format8 {
    fn from(value: u16) -> Self {
        let op = Opcode::from(value.get_bits(10, 11));
        let ro = value.get_bits_u8(6, 8);
        let rb = value.get_bits_u8(3, 5);
        let rd = value.get_bits_u8(0, 2);

        Self { op, ro, rb, rd }
    }
}

#[derive(Debug)]
enum Opcode {
    STRH,
    LDSB,
    LDSH,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::STRH,
            1 => Self::LDSB,
            2 | 3 => Self::LDSH,
            _ => unreachable!("invalid format 8 opcode: {value:b}"),
        }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format8(&mut self, instr: Format8) {
        let addr = self.get_reg(instr.rb) + self.get_reg(instr.ro);

        match instr.op {
            Opcode::STRH => self.strh(instr.rd, addr),
            Opcode::LDSB => self.ldsb(instr.rd, addr),
            Opcode::LDSH => self.ldsh(instr.rd, addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lds() {
        let asm = r"
            mov   r0, 10
            mov   r1, 11
            ldrsb r2, [r0, r0]
            ldrsh r3, [r0, r1]
        ";

        AsmTestBuilder::new()
            .thumb()
            .setup(|cpu| {
                cpu.bus.write_byte(20, -1_i8 as u8);
                cpu.bus.write_hword(21, -5_i16 as u16);
            })
            .asm(asm)
            .assert_reg(2, -1_i32 as u32)
            .assert_reg(3, -5_i32 as u32)
            .run(4);
    }
}
