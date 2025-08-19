use super::prelude::*;

/// Long branch with link
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  1 |  1 |  H |                      Offset                          |
/// +-------------------------------------------------------------------------------+
pub struct Format19 {
    h: bool,
    nn: u16,
}

impl Debug for Format19 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.h { "; BL label" } else { "BL label" })
    }
}

impl From<u16> for Format19 {
    fn from(value: u16) -> Self {
        let h = value.has(11);
        let nn = value.get_bits(0, 10);

        Self { h, nn }
    }
}

impl<B: Bus> Arm7tdmi<B> {
    pub fn exec_thumb_format19(&mut self, instr: Format19) {
        match instr.h {
            false => self.first_instr(instr.nn),
            true => self.second_instr(instr.nn),
        }
    }

    fn first_instr(&mut self, nn: u16) {
        let nn = ((nn as i32) << 21) >> 21; // sign-extend 11 bits
        let upper = (nn as u32) << 12;
        let result = self.pc().wrapping_add(upper);

        self.set_reg(Self::LR, result);
    }

    fn second_instr(&mut self, nn: u16) {
        let lower = (nn as u32) << 1;
        let lr = self.get_reg(Self::LR) as i32;
        let offset = lr.wrapping_add(lower as i32);
        let lr = self.next_instr_addr().unwrap_or_default() | 1;

        self.set_pc(offset as u32);
        self.set_reg(Self::LR, lr);

        self.pipeline.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looooong_branch() {
        let asm = r"
            main:
                bl  target ; 0-2
                mov r0, #1 ; 4

            last:
                mov r2, #4 ; 6

            target:
                mov r1, #2 ; 8
                bl last    ; 10-12
        ";

        AsmTestBuilder::new()
            .thumb()
            .asm(asm)
            .assert_reg(0, 0)
            .assert_reg(1, 2)
            .assert_reg(2, 4)
            .assert_reg(15, 12) // + pre-fetch 6
            .run(6)
    }
}
