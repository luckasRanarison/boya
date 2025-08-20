use super::prelude::*;

/// Software interrupt
/// +-------------------------------------------------------------------------------+
/// | 15 | 14 | 13 | 12 | 11 | 10 | 09 | 08 | 07 | 06 | 05 | 04 | 03 | 02 | 01 | 00 |
/// |-------------------------------------------------------------------------------|
/// |  1 |  1 |  0 |  1 |  1 |  1 |  1 |  1 |                 Value8                |
/// +-------------------------------------------------------------------------------+
pub struct Format17 {
    nn: u8,
}

impl Debug for Format17 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SWI {:?}", self.nn)
    }
}

impl From<u16> for Format17 {
    fn from(value: u16) -> Self {
        let nn = value.get_bits_u8(0, 7);

        Self { nn }
    }
}

impl<B: Bus> Executable<B> for Format17 {
    // the immediate parameter is only used by the exception handler
    fn dispatch(self, cpu: &mut Arm7tdmi<B>) -> Cycle {
        cpu.swi()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_swi() {
    //     AsmTestBuilder::new()
    //         .thumb()
    //         .asm("swi #7")
    //         .prg_offset(100)
    //         .setup(|cpu| {
    //             cpu.bus.write_word(0x08, 0);
    //             cpu.set_pc(100);
    //         })
    //         .assert_reg(2, 2)
    //         .run(4)
    // }
}
